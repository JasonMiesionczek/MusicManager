use crate::map;
use data::{models::*, repos::{AlbumRepository, ArtistRepository, Repository, SongRepository}};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct LibrarySong {
    pub id: u32,
    pub name: String,
    pub track: u32,
    pub filename: String,
}

impl From<Song> for LibrarySong {
    fn from(song: Song) -> Self {
        LibrarySong {
            id: song.id,
            name: song.name,
            track: song.track_num,
            filename: song.filename,
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct LibraryAlbum {
    pub id: u32,
    pub name: String,
    pub year: u32,
    pub external_id: String,
    pub artist: String,
    pub songs: Vec<LibrarySong>,
}

impl From<Album> for LibraryAlbum {
    fn from(album: Album) -> Self {
        LibraryAlbum {
            id: album.id,
            name: album.name,
            year: album.year,
            external_id: album.external_id,
            artist: String::from(""),
            songs: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct LibraryArtist {
    pub id: u32,
    pub name: String,
    pub external_id: String,
    pub albums: Vec<LibraryAlbum>,
}

impl From<Artist> for LibraryArtist {
    fn from(artist: Artist) -> Self {
        LibraryArtist {
            id: artist.id,
            name: artist.name,
            external_id: artist.external_id,
            albums: Vec::new(),
        }
    }
}

pub struct LibraryService {}

impl LibraryService {
    pub fn new() -> Self {
        LibraryService {}
    }

    pub fn get_album_songs(&self, album_id: &str, pool: &mysql::Pool) -> Vec<Song> {
        let song_repo = SongRepository::new();
        let songs = song_repo.find_by(crate::map! {"album_id" => album_id}, &pool);
        songs
            // .into_iter()
            // .map(|a| LibrarySong::from(a))
            // .collect::<Vec<LibrarySong>>()
    }

    pub fn get_artist_albums(&self, artist_id: &str, pool: &mysql::Pool) -> Vec<Album> {
        let album_repo = AlbumRepository::new();
        let albums = album_repo.find_by(crate::map! {"artist_id" => artist_id }, &pool);
        albums
            // .into_iter()
            // .map(|a| LibraryAlbum::from(a))
            // .collect::<Vec<LibraryAlbum>>()
    }

    pub fn get_artists(&self, pool: &mysql::Pool) -> Vec<Artist> {
        let artist_repo = ArtistRepository::new();
        artist_repo.get_all(&pool)
    }

    // pub fn get_all_music(&self, pool: &mysql::Pool) -> Vec<LibraryArtist> {
    //     let mut result: Vec<LibraryArtist> = Vec::new();
    //     let artist_repo = ArtistRepository::new();
    //     let album_repo = AlbumRepository::new();
    //     let song_repo = SongRepository::new();

    //     for artist in artist_repo.get_all(pool) {
    //         let artist_local = artist.clone();
    //         let mut library_artist = LibraryArtist::from(artist);

    //         let artist_local = artist.clone();
    //         let artist_id = artist.id.to_string();
    //         let artist_name = artist_local.name;

    //         for album in album_repo.find_by(map! { "artist_id" => artist_id.as_str() }, pool) {
    //             let mut library_album = LibraryAlbum::from(album);

    //             let album_id = album.id.to_string();

    //             for song in song_repo.find_by(map! { "album_id" => album_id.as_str() }, pool) {
    //                 library_album.songs.push(LibrarySong::from(song));
    //             }

    //             library_artist.albums.push(library_album);
    //         }

    //         result.push(library_artist);
    //     }

    //     result
    // }
}
