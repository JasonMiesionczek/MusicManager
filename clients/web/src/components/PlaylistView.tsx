import * as React from 'react';
import { Link, Route } from 'react-router-dom';
import { Playlist, PlaylistService } from '../services/PlaylistService';
import { PlaylistSongView } from './PlaylistSongView';

export class PlaylistView extends React.Component<{}, { playlists: Playlist[], name: string }> {
    private ps: PlaylistService;
    constructor(props: object) {
        super(props);
        this.state = {
            name: "",
            playlists: [],
        }
        this.ps = new PlaylistService();
    }

    public async componentDidMount() {
        await this.getPlaylist();
    }

    public render() {
        return (
            <>
                <div className="row">
                    <div className="col-sm-9 artist-header">
                        <h2>Playlists</h2>
                    </div>
                    <div className="col-sm-3">
                        <form className="form-inline" onSubmit={this.handleSubmit}>
                            <label className="sr-only">Name:</label>
                            <input type="text" value={this.state.name} onChange={this.handleChange} className="form-control-sm mb-2 mr-sm-2" placeholder="Playlist name" />

                            <input type="submit" value="Create" className="btn btn-secondary mb-2" />
                        </form>
                    </div>
                </div>
                <div className="row no-gutters">
                    <div className="col-sm-3">
                        <ul>
                            {this.state.playlists.map(list => (<li key={list.id}><Link to={`/playlists/${list.id}`}>{list.name}</Link></li>))}
                        </ul>
                    </div>
                    <div className="col-sm-9">
                        <Route path="/playlists/:playlistid" render={routerProps => (<PlaylistSongView {...routerProps} />)} />
                    </div>
                </div>
            </>
        )
    }

    private async getPlaylist() {
        const lists = await this.ps.getPlaylists();
        this.setState({ playlists: lists });
    }

    private handleChange = (event: any) => {
        this.setState({ name: event.target.value });
    }

    private handleSubmit = (event: any) => {
        event.preventDefault();
        this.ps.createPlaylist(this.state.name);
        this.getPlaylist();
    }
}