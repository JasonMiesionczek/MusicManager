export interface Artist {
  id: number;
  name: string;
  external_id: string;
}

export interface ArtistResult {
  artist: Artist;
  album_count: number;
}

export interface ArtistsResult {
  artists: ArtistResult[];
}

export interface Album {
  id: number;
  name: string;
  year: number;
  external_id: string;
}

export interface AlbumResult {
  albums: Album[],
  artist: Artist
}

export interface Song {
  id: number;
  name: string;
  filename: string;
  track_num: number;
}

export interface SongResult {
  artist: Artist;
  album: Album;
  songs: Song[];
}

export class LibraryService {
  public async getArtists(): Promise<ArtistsResult> {
    const response = await fetch("http://musicmanager.hopto.org:8000/api/library/artists");
    const json = await response.json();
    return json;
  }

  public async getAlbums(artistId: number): Promise<AlbumResult> {
    const response = await fetch(`http://musicmanager.hopto.org:8000/api/library/albums/${artistId}`);
    const json = await response.json();
    return json;
  }

  public async getSongs(albumId: number): Promise<SongResult> {
    const response = await fetch(`http://musicmanager.hopto.org:8000/api/library/songs/${albumId}`);
    const json = await response.json();
    return json;
  }
}
