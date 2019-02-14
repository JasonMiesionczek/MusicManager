use core::{map, services::YoutubeService};
use data::{
    models::*,
    repos::{AlbumRepository, ArtistRepository, Repository, SongRepository, TaskRepository},
};

use log::warn;
use mysql as my;
use std::collections::HashMap;

fn create_artist(artist_name: &str, pool: &my::Pool) -> Option<Artist> {
    let artist_repo = ArtistRepository::new();
    let artists = artist_repo.find_by(map! {"name" => artist_name}, pool);
    if artists.len() == 0 {
        let mut artist = Artist::new(artist_name.to_string());
        if let Err(msg) = artist_repo.create(&mut artist, pool) {
            warn!("{}", msg);
            None
        } else {
            create_task(TaskType::GetArtistImage(artist_name.to_string()), &pool);
            create_artist(artist_name, pool)
        }
    } else {
        artists.get(0).cloned()
    }
}

fn find_artist(artist_name: &str, pool: &my::Pool) -> Option<Artist> {
    let artist_repo = ArtistRepository::new();
    let artists = artist_repo.find_by(map! {"name" => artist_name}, pool);
    artists.get(0).cloned()
}

fn create_album(meta: &AlbumMeta, artist: Artist, pool: &my::Pool) -> Option<Album> {
    let album_repo = AlbumRepository::new();
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
    let song_repo = SongRepository::new();
    let album_id = album.id.to_string();
    let name = meta.name.replace("'", "''");
    let external_id = &meta.id;
    let songs = song_repo.find_by(
        map! {"name" => name.as_str(), "album_id"=> album_id.as_str(), "external_id" => external_id.as_str()},
        pool,
    );
    if songs.len() == 0 {
        let mut song = Song::new(meta.name.clone(), meta.num, album.id, meta.id.clone());
        if let Err(msg) = song_repo.create(&mut song, pool) {
            warn!("{}", msg);
        }
        Some(song)
    } else {
        songs.get(0).cloned()
    }
}

fn create_task(task_type: TaskType, pool: &my::Pool) {
    let task_repo = TaskRepository::new();
    let mut task = Task::new(task_type);
    task_repo.create(&mut task, &pool).unwrap();
}

fn update_task(task: &Task, status: TaskStatus, pool: &my::Pool) {
    let task_repo = TaskRepository::new();
    let mut values = HashMap::new();
    let status_string = status.to_string();
    values.insert("status", status_string.as_str());
    task_repo.update("tasks", values, task.id, &pool);
}

pub fn do_task(task: Task, pool: &my::Pool) {
    let youtube_service = YoutubeService::new();
    let album_repo = AlbumRepository::new();
    let song_repo = SongRepository::new();
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
            if albums.len() > 0 {
                for album in albums {
                    if let Some(artist) = create_artist(&album.artist, &pool) {
                        let _album = create_album(&album, artist, &pool);
                        let album_clone = album.clone();
                        create_task(TaskType::GetSongData(album), &pool);
                        create_task(TaskType::GetAlbumImage(album_clone), &pool);
                    }
                }
                update_task(&task, TaskStatus::Complete, &pool);
            } else {
                warn!("Task aborted. Retrying.");
                update_task(&task, TaskStatus::Pending, &pool);
            }
        }
        TaskType::GetAlbumImage(ref album_meta) => {
            update_task(&task, TaskStatus::InProgress, &pool);
            let meta = album_meta.clone();
            let id = meta.id;
            if let Some(_album) = album_repo.find_by_external_id(id, &pool) {
                let image_url = youtube_service.get_album_image_url(album_meta.id.as_str());
                youtube_service.download_image(album_meta.clone().id.as_str(), image_url.as_str());
            }
            update_task(&task, TaskStatus::Complete, &pool);
        }
        TaskType::GetArtistImage(ref artist_name) => {
            update_task(&task, TaskStatus::InProgress, &pool);
            let image_url = youtube_service.get_artist_image_url(artist_name.as_str());
            if let Some(artist) = find_artist(artist_name, &pool) {
                youtube_service.download_image(artist.external_id.as_str(), image_url.as_str());
            }
            update_task(&task, TaskStatus::Complete, &pool);
        }
        TaskType::GetSongData(ref album_meta) => {
            update_task(&task, TaskStatus::InProgress, &pool);
            let meta = album_meta.clone();
            if let Some(album) = album_repo.find_by_external_id(meta.id, &pool) {
                let songs = youtube_service.get_track_list(album_meta.id.clone().as_str());
                if songs.len() > 0 {
                    for song in songs {
                        let _song = create_song(&song, &album, &pool);
                        create_task(TaskType::DownloadSong(song), &pool);
                    }
                    update_task(&task, TaskStatus::Complete, &pool);
                } else {
                    warn!("Task aborted. Retrying.");
                    update_task(&task, TaskStatus::Pending, &pool);
                }
            } else {
                update_task(&task, TaskStatus::Complete, &pool);
            }
        }
        TaskType::DownloadSong(ref song_meta) => {
            update_task(&task, TaskStatus::InProgress, &pool);
            let meta = song_meta.clone();
            if let Some(album) = album_repo.find_by_external_id(meta.album_id, &pool) {
                let album_id = album.id.to_string();
                if let Some(song) = song_repo.find_one_by(
                    map! { "external_id" => meta.id.as_str(), "album_id" => album_id.as_str()},
                    &pool,
                ) {
                    if youtube_service
                        .download_song(song.external_id.as_str(), song.filename.as_str())
                    {
                        update_task(&task, TaskStatus::Complete, &pool);
                        create_task(TaskType::GenerateWaveform(song_meta.clone()), &pool);
                    } else {
                        warn!("Task FAILED. Retrying.");
                        update_task(&task, TaskStatus::Pending, &pool);
                    }
                }
            }
        }
        TaskType::GenerateWaveform(ref song_meta) => {
            update_task(&task, TaskStatus::InProgress, &pool);
            let meta = song_meta.clone();
            if let Some(album) = album_repo.find_by_external_id(meta.album_id, &pool) {
                let album_id = album.id.to_string();
                if let Some(song) = song_repo.find_one_by(
                    map! { "external_id" => meta.id.as_str(), "album_id" => album_id.as_str()},
                    &pool,
                ) {
                    if youtube_service.generate_waveform(song.filename.as_str()) {
                        update_task(&task, TaskStatus::Complete, &pool);
                    } else {
                        warn!("Task FAILED. Retrying.");
                        update_task(&task, TaskStatus::Pending, &pool);
                    }
                }
            }
        }
    }
}
