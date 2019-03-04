use core::{map, services::youtube::DownloadError, services::YoutubeService};
use data::{
    models::*,
    repos::{
        AlbumRepository, ArtistRepository, Repository, SongRepository, TaskRepository, UpdateValue,
    },
};
use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    NewTask(Task),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn execute_task(&self, task: Task) {
        self.sender.send(Message::NewTask(task)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate to all workers");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got job; executing.", id);
                    job.call_box();
                }
                Message::NewTask(task) => {
                    log::info!("Worker {} got job {}", id, task);
                    let task_manager = TaskManager::new();
                    task_manager.execute_task(&task);
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct TaskManager {
    db_pool: mysql::Pool,
    task_repo: TaskRepository,
    song_repo: SongRepository,
    artist_repo: ArtistRepository,
    album_repo: AlbumRepository,
    youtube_service: YoutubeService,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            db_pool: data::get_pool(),
            task_repo: TaskRepository::new(),
            song_repo: SongRepository::new(),
            artist_repo: ArtistRepository::new(),
            album_repo: AlbumRepository::new(),
            youtube_service: YoutubeService::new(),
        }
    }

    fn create_artist(&self, artist_name: &str) -> Option<Artist> {
        let artists = self
            .artist_repo
            .find_by(map! {"name" => artist_name}, &self.db_pool);
        if artists.len() == 0 {
            let mut artist = Artist::new(artist_name.to_string());
            if let Err(msg) = self.artist_repo.create(&mut artist, &self.db_pool) {
                warn!("{}", msg);
                None
            } else {
                self.create_task(TaskType::GetArtistImage(artist_name.to_string()));
                self.create_artist(artist_name)
            }
        } else {
            artists.get(0).cloned()
        }
    }

    fn find_artist(&self, artist_name: &str) -> Option<Artist> {
        let artists = self
            .artist_repo
            .find_by(map! {"name" => artist_name}, &self.db_pool);
        artists.get(0).cloned()
    }

    fn create_album(&self, meta: &AlbumMeta, artist: Artist) -> Option<Album> {
        let artist_id = artist.id.to_string();
        let external_id = &meta.id;
        let name = meta.name.replace("'", "''");
        let albums = self.album_repo.find_by(
            map! {"name" => name.as_str(), "artist_id" => artist_id.as_str(), "external_id" => external_id.as_str()},
            &self.db_pool,
        );
        if albums.len() == 0 {
            let mut album = Album::new(
                meta.name.clone(),
                meta.year.parse::<u32>().unwrap(),
                meta.image.clone(),
                artist.id,
                meta.id.clone(),
            );
            self.album_repo.create(&mut album, &self.db_pool).unwrap();
            Some(album)
        } else {
            albums.get(0).cloned()
        }
    }

    fn create_song(&self, meta: &SongMeta, album: &Album) -> Option<Song> {
        let album_id = album.id.to_string();
        let name = meta.name.replace("'", "''");
        let external_id = &meta.id;
        let songs = self.song_repo.find_by(
            map! {"name" => name.as_str(), "album_id"=> album_id.as_str(), "external_id" => external_id.as_str()},
            &self.db_pool,
        );
        if songs.len() == 0 {
            let mut song = Song::new(meta.name.clone(), meta.num, album.id, meta.id.clone());
            if let Err(msg) = self.song_repo.create(&mut song, &self.db_pool) {
                warn!("{}", msg);
            }
            Some(song)
        } else {
            songs.get(0).cloned()
        }
    }

    fn create_task(&self, task_type: TaskType) {
        let mut task = Task::new(task_type);
        self.task_repo.create(&mut task, &self.db_pool).unwrap();
    }

    fn update_task(&self, task: &Task, status: TaskStatus) {
        let mut values = HashMap::new();
        let status_string = status.to_string();
        info!("Task {}: {}", task, status_string);
        values.insert("status", UpdateValue::Str(status_string));
        self.task_repo
            .update("tasks", values, task.id, &self.db_pool);
    }

    fn retry_or_fail(&self, task: &Task) {
        if task.retry_count < 3 {
            let new_count = task.retry_count + 1;
            let mut values = HashMap::new();
            values.insert("retry_count", UpdateValue::Int(new_count));
            values.insert("status", UpdateValue::Str(String::from("pending")));
            warn!("Task {}: Failed, try #{}", task, new_count);
            self.task_repo
                .update("tasks", values, task.id, &self.db_pool);
        } else {
            error!("Task {}: Max retry count hit. Failed.", task);
            self.update_task(&task, TaskStatus::Failed);
        }
    }

    fn handle_get_album_data(&self, task: &Task, artist_name: String) {
        self.update_task(&task, TaskStatus::InProgress);
        if let Ok(albums) = self.youtube_service.get_album_data(&artist_name) {
            if albums.len() > 0 {
                for album in albums {
                    if let Some(artist) = self.create_artist(&album.artist) {
                        let _album = self.create_album(&album, artist);
                        let album_clone = album.clone();
                        self.create_task(TaskType::GetSongData(album));
                        self.create_task(TaskType::GetAlbumImage(album_clone));
                    }
                }
                self.update_task(&task, TaskStatus::Complete);
            } else {
                self.retry_or_fail(&task);
            }
        }
    }

    fn handle_get_album_image(&self, task: &Task, album_meta: &AlbumMeta) {
        self.update_task(&task, TaskStatus::InProgress);
        let meta = album_meta.clone();
        let id = meta.id;
        if let Some(_album) = self.album_repo.find_by_external_id(id, &self.db_pool) {
            if let Ok(image_url) = self
                .youtube_service
                .get_album_image_url(album_meta.id.as_str())
            {
                if let Ok(_) = self
                    .youtube_service
                    .download_image(album_meta.clone().id.as_str(), image_url.as_str())
                {
                    self.update_task(&task, TaskStatus::Complete);
                } else {
                    self.retry_or_fail(&task);
                }
            }
        }
    }

    fn handle_get_artist_image(&self, task: &Task, artist_name: String) {
        self.update_task(&task, TaskStatus::InProgress);
        if let Ok(image_url) = self
            .youtube_service
            .get_artist_image_url(artist_name.as_str())
        {
            if let Some(artist) = self.find_artist(artist_name.as_str()) {
                if let Ok(_) = self
                    .youtube_service
                    .download_image(artist.external_id.as_str(), image_url.as_str())
                {
                    self.update_task(&task, TaskStatus::Complete);
                } else {
                    self.retry_or_fail(&task);
                }
            }
        }
    }

    fn handle_get_song_data(&self, task: &Task, album_meta: &AlbumMeta) {
        self.update_task(&task, TaskStatus::InProgress);
        let meta = album_meta.clone();
        if let Some(album) = self.album_repo.find_by_external_id(meta.id, &self.db_pool) {
            if let Ok(songs) = self
                .youtube_service
                .get_track_list(album_meta.id.clone().as_str())
            {
                if songs.len() > 0 {
                    for song in songs {
                        let _song = self.create_song(&song, &album);
                        self.create_task(TaskType::DownloadSong(song));
                    }
                    self.update_task(&task, TaskStatus::Complete);
                } else {
                    self.retry_or_fail(&task);
                }
            }
        } else {
            self.update_task(&task, TaskStatus::Complete);
        }
    }

    fn handle_download_song(&self, task: &Task, song_meta: &SongMeta) {
        self.update_task(&task, TaskStatus::InProgress);
        let meta = song_meta.clone();
        if let Some(album) = self
            .album_repo
            .find_by_external_id(meta.album_id, &self.db_pool)
        {
            let album_id = album.id.to_string();
            if let Some(song) = self.song_repo.find_one_by(
                map! { "external_id" => meta.id.as_str(), "album_id" => album_id.as_str()},
                &self.db_pool,
            ) {
                match self
                    .youtube_service
                    .download_song(song.external_id.as_str(), song.filename.as_str())
                {
                    Ok(_) => {
                        self.update_task(&task, TaskStatus::Complete);
                        self.create_task(TaskType::GenerateWaveform(song_meta.clone()));
                    }
                    Err(err) => match err {
                        DownloadError::NotAvailable => {
                            self.update_task(&task, TaskStatus::Failed);
                        }
                        DownloadError::Forbidden => {
                            self.retry_or_fail(&task);
                        }
                        DownloadError::Unspecified(msg) => {
                            warn!("{}", msg);
                            self.retry_or_fail(&task);
                        }
                    },
                }
            }
        }
    }

    fn handle_generate_waveform(&self, task: &Task, song_meta: &SongMeta) {
        self.update_task(&task, TaskStatus::InProgress);
        let meta = song_meta.clone();
        if let Some(album) = self
            .album_repo
            .find_by_external_id(meta.album_id, &self.db_pool)
        {
            let album_id = album.id.to_string();
            if let Some(song) = self.song_repo.find_one_by(
                map! { "external_id" => meta.id.as_str(), "album_id" => album_id.as_str()},
                &self.db_pool,
            ) {
                if let Ok(_) = self
                    .youtube_service
                    .generate_waveform(song.filename.as_str())
                {
                    self.update_task(&task, TaskStatus::Complete);
                } else {
                    self.retry_or_fail(&task);
                }
            }
        }
    }

    pub fn execute_task(&self, task: &Task) {
        match task.task_type {
            TaskType::GetAlbumData(ref artist_name) => {
                self.handle_get_album_data(&task, artist_name.to_string());
            }
            TaskType::GetAlbumImage(ref album_meta) => {
                self.handle_get_album_image(&task, album_meta);
            }
            TaskType::GetArtistImage(ref artist_name) => {
                self.handle_get_artist_image(&task, artist_name.to_string());
            }
            TaskType::GetSongData(ref album_meta) => {
                self.handle_get_song_data(&task, album_meta);
            }
            TaskType::DownloadSong(ref song_meta) => {
                self.handle_download_song(&task, song_meta);
            }
            TaskType::GenerateWaveform(ref song_meta) => {
                self.handle_generate_waveform(&task, song_meta);
            }
        }
    }
}
