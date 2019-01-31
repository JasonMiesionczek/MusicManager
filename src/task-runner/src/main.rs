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

fn get_pool() -> my::Pool {
    let url = dotenv::var("DATABASE_URL").expect("could not find DATABASE_URL");
    let pool = my::Pool::new(url).unwrap();
    pool
}

fn create_artist(artist_name: &str, pool: &my::Pool) -> Artist {
    let artist_repo = ArtistRepository {};
    let artists = artist_repo.find_by(map! {"name" => artist_name}, pool);
    if artists.len() == 0 {
        let mut artist = Artist::new(artist_name.to_string());
        artist_repo.create(&mut artist, pool).unwrap();
        create_artist(artist_name, pool)
    } else {
        artists.get(0).cloned().unwrap()
    }
}

fn create_album(meta: &AlbumMeta, artist: Artist, pool: &my::Pool) -> Album {
    let album_repo = AlbumRepository {};
    let artist_id = artist.id.to_string();
    let external_id = &meta.id;
    let albums = album_repo.find_by(
        map! {"name" => meta.name.as_str(), "artist_id" => artist_id.as_str(), "external_id" => external_id.as_str()},
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
        album
    } else {
        albums.get(0).cloned().unwrap()
    }
}

fn create_song(meta: &SongMeta, album: &Album, pool: &my::Pool) -> Song {
    let song_repo = SongRepository {};
    let album_id = album.id.to_string();
    let name = meta.name.replace("'", "''");
    let songs = song_repo.find_by(
        map! {"name" => name.as_str(), "album_id"=> album_id.as_str()},
        pool,
    );
    if songs.len() == 0 {
        let mut song = Song::new(meta.name.clone(), meta.num, album.id, meta.id.clone());
        song_repo.create(&mut song, pool).unwrap();
        song
    } else {
        songs.get(0).cloned().unwrap()
    }
}

fn update_task(task: &Task, status: TaskStatus) {
    let task_repo = TaskRepository {};
    let mut values = HashMap::new();
    let status_string = status.to_string();
    values.insert("status", status_string.as_str());
    task_repo.update("tasks", values, task.id, get_pool());
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
            update_task(&task, TaskStatus::InProgress);
            let albums = youtube_service.get_album_data(&artist_name);
            for album in albums {
                let artist = create_artist(&album.artist, &pool);
                println!("Artist: {:?}", artist);
                let _album = create_album(&album, artist, &pool);
                let mut task = Task::new(TaskType::GetSongData(album));
                task_repo.create(&mut task, &pool).unwrap();
            }
            update_task(&task, TaskStatus::Complete);
        }
        TaskType::GetSongData(ref album_meta) => {
            update_task(&task, TaskStatus::InProgress);
            let meta = album_meta.clone();
            let album = album_repo.find_by_external_id(meta.id, &pool);
            let songs = youtube_service.get_track_list(album_meta.id.clone().as_str());
            for song in songs {
                let _song = create_song(&song, &album, &pool);
                let mut task = Task::new(TaskType::DownloadSong(song));
                task_repo.create(&mut task, &pool).unwrap();
            }
            update_task(&task, TaskStatus::Complete);
        }
        TaskType::DownloadSong(ref song_meta) => {
            update_task(&task, TaskStatus::InProgress);
            let meta = song_meta.clone();
            let song = song_repo.find_by_external_id(meta.id, &pool);
            youtube_service.download_song(song.external_id.as_str(), song.filename.as_str());
            update_task(&task, TaskStatus::Complete);
        }
    }
}

fn main() {
    dotenv().ok();
    let task_repo = TaskRepository {};
    let one_minute = time::Duration::from_secs(60);
    let pool = get_pool();
    loop {
        let tasks = task_repo
            .get_all(&pool)
            .into_iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .collect::<Vec<_>>();
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
