use musiccore::services::{youtube::YoutubeService, LibraryService};
use data::{
    models::*,
    repos::{Repository, TaskRepository},
};
use dotenv;
use id3::{
    frame::{Picture, PictureType},
    Tag, Version,
};
use log::{error, info, warn};
use mysql as my;
use prettytable::{cell, format, row, Table};
use std::fs::create_dir_all;
use std::fs::File;
use std::io::prelude::*;

fn get_pool() -> my::Pool {
    let url = dotenv::var("DATABASE_URL").expect("could not find DATABASE_URL");
    let pool = my::Pool::new(url).unwrap();
    pool
}

pub fn search_command(artist_name: &str) {
    let ms = YoutubeService::new();
    if let Ok(albums) = ms.get_album_data(artist_name) {
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new('-', '+', '+', '+'),
            )
            .padding(1, 1)
            .build();
        table.set_format(format);

        table.set_titles(row![bFg -> "Album", bFg -> "Year"]);
        for album in &albums {
            table.add_row(row![album.name, album.year]);
        }

        table.printstd();
    }
}

pub fn queue_download(artist_name: &str) {
    let task_repo = TaskRepository {};
    let pool = get_pool();
    let mut task = Task::new(TaskType::GetAlbumData(artist_name.to_string()));
    task_repo.create(&mut task, &pool).unwrap();
}

pub fn export_music(dir: &str) {
    info!("Creating directory: {}", dir);
    create_dir_all(dir).unwrap();
    let music_dir = dotenv::var("MUSIC_DOWNLOAD_DIR").expect("download directory not specified");
    let image_dir =
        dotenv::var("IMAGE_DOWNLOAD_DIR").expect("image download directory not specified");
    let ls = LibraryService::new();
    let pool = get_pool();
    for artist in ls.get_artists(&pool) {
        let artist_folder = format!("{}/{}", dir, artist.name);
        info!("Artist folder: {}", artist_folder);
        for album in ls.get_artist_albums(artist.id.to_string().as_str(), &pool) {
            let album_folder = &format!("{}/{}", artist_folder, album.name);
            info!("Album folder: {}", album_folder);
            create_dir_all(album_folder).unwrap();
            for song in ls.get_album_songs(album.id.to_string().as_str(), &pool) {
                let source_file = format!("{}/{}.mp3", music_dir, song.filename);
                let output_file = format!("{}/{}.mp3", album_folder, song.name);
                if let Err(err) = std::fs::copy(source_file, &output_file) {
                    error!("{}", err);
                    continue;
                }
                let mut tag = Tag::new();
                tag.set_artist(artist.clone().name);
                tag.set_album_artist(artist.clone().name);
                tag.set_album(album.clone().name);
                tag.set_title(song.clone().name);
                tag.set_track(song.clone().track_num);
                tag.set_year(album.year as i32);
                match File::open(format!("{}/{}.jpg", image_dir, album.external_id)) {
                    Ok(mut f) => {
                        let mut album_image_data = Vec::new();
                        f.read_to_end(&mut album_image_data).unwrap();

                        tag.add_picture(Picture {
                            mime_type: "image/jpeg".to_string(),
                            picture_type: PictureType::Other,
                            description: "".to_string(),
                            data: album_image_data.clone(),
                        });

                        tag.add_picture(Picture {
                            mime_type: "image/jpeg".to_string(),
                            picture_type: PictureType::CoverFront,
                            description: "".to_string(),
                            data: album_image_data.clone(),
                        });
                    }
                    _ => {}
                };
                match tag.write_to_path(output_file, Version::Id3v24) {
                    Ok(_) => {
                        info!(
                            "Updated and Exported: {}/{}/{}/{}.mp3",
                            album_folder,
                            artist.name,
                            album.name,
                            song.clone().name
                        );
                    }
                    _ => {
                        warn!(
                            "Could not update: {}/{}/{}/{}.mp3, not found.",
                            album_folder,
                            artist.name,
                            album.name,
                            song.clone().name
                        );
                    }
                }
            }
        }
    }
}

pub fn update_music() {
    let music_dir = dotenv::var("MUSIC_DOWNLOAD_DIR").expect("download directory not specified");
    let image_dir =
        dotenv::var("IMAGE_DOWNLOAD_DIR").expect("image download directory not specified");
    let ls = LibraryService::new();
    let pool = get_pool();
    for artist in ls.get_artists(&pool) {
        for album in ls.get_artist_albums(artist.id.to_string().as_str(), &pool) {
            for song in ls.get_album_songs(album.id.to_string().as_str(), &pool) {
                let mut tag = Tag::new();
                tag.set_artist(artist.clone().name);
                tag.set_album_artist(artist.clone().name);
                tag.set_album(album.clone().name);
                tag.set_title(song.clone().name);
                tag.set_track(song.clone().track_num);
                tag.set_year(album.year as i32);
                // match File::open(format!("{}/{}.jpg", image_dir, artist.external_id)) {
                //     Ok(mut f) => {
                //         let mut artist_image_data = Vec::new();
                //         f.read_to_end(&mut artist_image_data).unwrap();
                //         tag.add_picture(Picture {
                //             mime_type: "image/jpeg".to_string(),
                //             picture_type: PictureType::Artist,
                //             description: "".to_string(),
                //             data: artist_image_data.clone(),
                //         });

                //         tag.add_picture(Picture {
                //             mime_type: "image/jpeg".to_string(),
                //             picture_type: PictureType::Band,
                //             description: "".to_string(),
                //             data: artist_image_data,
                //         });
                //     }
                //     _ => {}
                // };

                match File::open(format!("{}/{}.jpg", image_dir, album.external_id)) {
                    Ok(mut f) => {
                        let mut album_image_data = Vec::new();
                        f.read_to_end(&mut album_image_data).unwrap();

                        tag.add_picture(Picture {
                            mime_type: "image/jpeg".to_string(),
                            picture_type: PictureType::Other,
                            description: "".to_string(),
                            data: album_image_data.clone(),
                        });

                        tag.add_picture(Picture {
                            mime_type: "image/jpeg".to_string(),
                            picture_type: PictureType::CoverFront,
                            description: "".to_string(),
                            data: album_image_data.clone(),
                        });
                    }
                    _ => {}
                };
                //let song_filename = format!("{}/{}", music_dir, song.filename);
                //let path = Path::new(&song_filename);
                //let duration = mp3_duration::from_path(&path).unwrap();
                //tag.set_duration(duration.as_secs() as u32);
                match tag.write_to_path(
                    format!("{}/{}.mp3", music_dir, song.filename),
                    Version::Id3v24,
                ) {
                    Ok(_) => {
                        info!(
                            "Updated: {} - {} - {}",
                            artist.name,
                            album.name,
                            song.clone().name
                        );
                    }
                    _ => {
                        warn!(
                            "Could not update: {} - {} - {}, mp3 not found.",
                            artist.name,
                            album.name,
                            song.clone().name
                        );
                    }
                }
            }
        }
    }
}
