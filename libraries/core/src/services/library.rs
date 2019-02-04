use crate::map;
use data::repos::{AlbumRepository, ArtistRepository, Repository, SongRepository};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct LibrarySong {
    pub id: u32,
    pub name: String,
    pub track: u32,
    pub filename: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct LibraryAlbum {
    pub id: u32,
    pub name: String,
    pub year: u32,
    pub external_id: String,
    pub songs: Vec<LibrarySong>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct LibraryArtist {
    pub id: u32,
    pub name: String,
    pub external_id: String,
    pub albums: Vec<LibraryAlbum>,
}

pub struct LibraryService {}

impl LibraryService {
    pub fn new() -> Self {
        LibraryService {}
    }

    pub fn get_all_music(&self, pool: &mysql::Pool) -> Vec<LibraryArtist> {
        let mut result: Vec<LibraryArtist> = Vec::new();
        let artist_repo = ArtistRepository::new();
        let album_repo = AlbumRepository::new();
        let song_repo = SongRepository::new();

        for artist in artist_repo.get_all(pool) {
            let mut library_artist = LibraryArtist {
                id: artist.id,
                name: artist.name,
                external_id: artist.external_id,
                albums: Vec::new(),
            };

            let artist_id = artist.id.to_string();

            for album in album_repo.find_by(map! { "artist_id" => artist_id.as_str() }, pool) {
                let mut library_album = LibraryAlbum {
                    id: album.id,
                    name: album.name,
                    year: album.year,
                    external_id: album.external_id,
                    songs: Vec::new(),
                };

                let album_id = album.id.to_string();

                for song in song_repo.find_by(map! { "album_id" => album_id.as_str() }, pool) {
                    let library_song = LibrarySong {
                        id: song.id,
                        name: song.name,
                        track: song.track_num,
                        filename: song.filename,
                    };

                    library_album.songs.push(library_song);
                }

                library_artist.albums.push(library_album);
            }

            result.push(library_artist);
        }

        result
    }
}
