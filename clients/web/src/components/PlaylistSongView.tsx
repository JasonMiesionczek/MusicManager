import * as React from 'react';
import { Song } from '../services/LibraryService';
import { PlaylistService } from '../services/PlaylistService';

interface PlaylistSongViewProps {
    match: {
        params: {
            playlistid: number;
        }
    }
}

export class PlaylistSongView extends React.Component<PlaylistSongViewProps, { songs: Song[] }> {
    constructor(props: PlaylistSongViewProps) {
        super(props);
        this.state = {
            songs: []
        }
    }

    public async componentWillUpdate() {
        const ps = new PlaylistService();
        const songs = await ps.getPlaylistSongs(this.props.match.params.playlistid);
        this.setState({ songs });
    }

    public render() {
        return (
            <ul>
                {this.state.songs.map((song, idx) => <li key={idx}>{song.name}</li>)}
            </ul>
        )
    }
}