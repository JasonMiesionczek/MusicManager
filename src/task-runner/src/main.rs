use core::{map, services::youtube::YoutubeService};
use data::{
    models::*,
    repos::{
        album::AlbumRepository, artist::ArtistRepository, song::SongRepository,
        task::TaskRepository, Repository,
    },
};
use dotenv::dotenv;
use mysql as my;
use std::collections::HashMap;
use std::{thread, time};

fn create_artist(artist_name: &str, pool: &my::Pool) -> Option<Artist> {
    let artist_repo = ArtistRepository {};
    let artists = artist_repo.find_by(map! {"name" => artist_name}, pool);
    if artists.len() == 0 {
        let mut artist = Artist::new(artist_name.to_string());
        artist_repo.create(&mut artist, pool).unwrap();
        create_artist(artist_name, pool)
    } else {
        artists.get(0).cloned()
    }
}

fn create_album(meta: &AlbumMeta, artist: Artist, pool: &my::Pool) -> Option<Album> {
    let album_repo = AlbumRepository {};
    let artist_id = artist.id.to_string();
    let external_id = &meta.id;
    let name = meta.name.replace("'", "''");
    let albums = album_repo.find_by(
        map! {"name" => name.as_str(), "artist_id" => artist_id.as_str(), "external_id" => external_id.as_str()},
        pool,
    );
    if albums.len() == 0 {
        let mut album = Album::new(
            meta.name.clone(),
            meta.year.parse::<u32>().unwrap(),
            meta.image.clone(),
            artist.id,
            meta.id.clone(),
        );
        album_repo.create(&mut album, pool).unwrap();
        Some(album)
    } else {
        albums.get(0).cloned()
    }
}

fn create_song(meta: &SongMeta, album: &Album, pool: &my::Pool) -> Option<Song> {
    let song_repo = SongRepository {};
    let album_id = album.id.to_string();
    let name = meta.name.replace("'", "''");
    let external_id = &meta.id;
    let songs = song_repo.find_by(
        map! {"name" => name.as_str(), "album_id"=> album_id.as_str(), "external_id" => external_id.as_str()},
        pool,
    );
    if songs.len() == 0 {
        let mut song = Song::new(meta.name.clone(), meta.num, album.id, meta.id.clone());
        song_repo.create(&mut song, pool).unwrap();
        Some(song)
    } else {
        songs.get(0).cloned()
    }
}

fn update_task(task: &Task, status: TaskStatus, pool: &my::Pool) {
    let task_repo = TaskRepository {};
    let mut values = HashMap::new();
    let status_string = status.to_string();
    values.insert("status", status_string.as_str());
    task_repo.update("tasks", values, task.id, &pool);
}

fn do_task(task: Task, pool: &my::Pool) {
    let youtube_service = YoutubeService::new();
    let task_repo = TaskRepository {};
    let album_repo = AlbumRepository {};
    let song_repo = SongRepository {};
    match task.task_type {
        TaskType::GetAlbumData(ref artist_name) => {
            /*
                1) For each album we check the artist field, and create or get the artist
                    record.
                2) For each album we check if the album for that artist exists,
                    if not we create it
            */
            update_task(&task, TaskStatus::InProgress, &pool);
            let albums = youtube_service.get_album_data(&artist_name);
            for album in albums {
                if let Some(artist) = create_artist(&album.artist, &pool) {
                    let _album = create_album(&album, artist, &pool);
                    let album_clone = album.clone();
                    let mut task = Task::new(TaskType::GetSongData(album));
                    task_repo.create(&mut task, &pool).unwrap();
                    let mut image_task = Task::new(TaskType::GetAlbumImage(album_clone));
                    task_repo.create(&mut image_task, &pool).unwrap();
                }
            }
            update_task(&task, TaskStatus::Complete, &pool);
        }
        TaskType::GetAlbumImage(ref album_meta) => {
            update_task(&task, TaskStatus::InProgress, &pool);
            let meta = album_meta.clone();
            let id = meta.id;
            if let Some(_album) = album_repo.find_by_external_id(id, &pool) {
                let image_url = youtube_service.get_album_image_url(album_meta.id.as_str());
                println!("IMAGE URL: {}", image_url);
                youtube_service.download_image(album_meta.clone().id.as_str(), image_url.as_str());
            }
            update_task(&task, TaskStatus::Complete, &pool);
        }
        TaskType::GetSongData(ref album_meta) => {
            update_task(&task, TaskStatus::InProgress, &pool);
            let meta = album_meta.clone();
            if let Some(album) = album_repo.find_by_external_id(meta.id, &pool) {
                let songs = youtube_service.get_track_list(album_meta.id.clone().as_str());
                for song in songs {
                    let _song = create_song(&song, &album, &pool);
                    let mut task = Task::new(TaskType::DownloadSong(song));
                    task_repo.create(&mut task, &pool).unwrap();
                }
                update_task(&task, TaskStatus::Complete, &pool);
            } else {
                update_task(&task, TaskStatus::Complete, &pool);
            }
        }
        TaskType::DownloadSong(ref song_meta) => {
            update_task(&task, TaskStatus::InProgress, &pool);
            let meta = song_meta.clone();
            if let Some(song) = song_repo.find_by_external_id(meta.id, &pool) {
                youtube_service.download_song(song.external_id.as_str(), song.filename.as_str());
                update_task(&task, TaskStatus::Complete, &pool);
            }
        }
    }
}

fn main() {
    dotenv().ok();
    let task_repo = TaskRepository {};
    let one_minute = time::Duration::from_secs(60);
    let pool = data::get_pool();
    loop {
        let tasks = task_repo.find_by(core::map! {"status" => "pending"}, &pool);
        println!("Found {} tasks", tasks.len());
        for task in tasks {
            match task.status {
                TaskStatus::Pending => {
                    println!("Processing task {}: {:?}", task.id, task.task_type);
                    do_task(task, &pool);
                }
                _ => {}
            }
        }
        println!("Sleeping one minute");
        thread::sleep(one_minute);
    }
}
