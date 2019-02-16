import * as $ from "jquery";
import { Song } from './LibraryService';

export interface Playlist {
    id: number;
    name: string;
}

export class PlaylistService {
    public async getPlaylists(): Promise<Playlist[]> {
        const response = await fetch('/api/playlists');
        const data = await response.json();
        return data;
    }

    public async createPlaylist(name: string) {
        await $.ajax('/api/playlist', { data: JSON.stringify({ "name": name }), contentType: 'application/json', type: 'POST' }).promise();
    }

    public async addSongToPlaylist(playlistId: number, songId: number) {
        await $.ajax(
            '/api/playlist_song',
            {
                contentType: 'application/json',
                data: JSON.stringify({ "playlist_id": playlistId, "song_id": songId }),
                type: 'POST',
            }
        ).promise();
    }

    public async getPlaylistSongs(playlistId: number): Promise<Song[]> {
        const response = await fetch(`/api/playlist/${playlistId}`);
        const data = await response.json();
        return data;
    }
}