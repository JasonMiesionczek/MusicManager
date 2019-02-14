import * as $ from 'jquery';
import * as React from 'react';
import { Link } from 'react-router-dom';
import { LibraryService, SongResult } from '../services/LibraryService';

interface SongViewProps {
    match: {
        params: {
            album_id: number
        }
    };
    playSongHandler: any;
}

export class SongView extends React.Component<SongViewProps, { data: SongResult, isLoading: boolean }> {
    constructor(props: SongViewProps) {
        super(props);
        this.state = {
            data: {
                album: {
                    external_id: "",
                    id: 0,
                    name: "",
                    year: 0
                },
                artist: {
                    external_id: "",
                    id: 0,
                    name: ""
                },
                songs: []
            },
            isLoading: true
        }
    }

    public async componentDidMount() {
        const ls = new LibraryService();
        const result = await ls.getSongs(this.props.match.params.album_id);
        this.setState({ data: result, isLoading: false });
    }

    public render() {
        if (this.state.isLoading) {
            return (<div>Loading...</div>)
        } else {
            return (
                <React.Fragment>
                    <div className="row">
                        <div className="col-sm-12 artist-header">
                            <h2><Link to="/artists">Artists</Link> <i className="fas fa-chevron-right" /> <Link to={`/artist/${this.state.data.artist.id}`}>{this.state.data.artist.name}</Link> <i className="fas fa-chevron-right" /> {this.state.data.album.name}</h2>

                        </div>
                    </div>
                    <div className="row no-gutters">
                        {this.state.data.songs.map(song => (
                            <div className="col-xs-12 col-sm-12" key={song.id} onClick={this.props.playSongHandler.bind(this, song, this.state.data.album, this.state.data.artist)} onMouseEnter={this.onHover} onMouseLeave={this.onOut} style={{ height: 100 + "px", backgroundSize: "1024px 100px", backgroundRepeat: "no-repeat", backgroundPosition: "right", backgroundImage: `url(http://localhost:90/images/${song.filename}.png)` }}>
                                <div className="song-card">
                                    <h5>{song.track_num}</h5>
                                    <h3>{song.name}</h3>
                                </div>
                            </div>
                        ))}
                    </div>
                </React.Fragment>
            );
        }

    }

    private onHover = (event: React.MouseEvent<HTMLElement>) => {
        $(event.currentTarget).find('.song-card').addClass('song-card-hover');
        // $(event.currentTarget).addClass('grow').removeClass('shrink');
    };

    private onOut = (event: React.MouseEvent<HTMLElement>) => {
        $(event.currentTarget).find('.song-card').removeClass('song-card-hover');
        // $(event.currentTarget).addClass('shrink').removeClass('grow');
    }
}