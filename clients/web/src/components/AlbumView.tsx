import { History } from 'history';
import * as $ from 'jquery';
import * as React from 'react';
import { Link } from 'react-router-dom';
import { AlbumResult, LibraryService } from '../services/LibraryService';
import "./AlbumView.css";

interface AlbumViewProps {
    match: {
        params: {
            artist_id: number
        }
    };
    history: History;
}

export class AlbumView extends React.Component<AlbumViewProps, { data: AlbumResult, isLoading: boolean }> {
    constructor(props: AlbumViewProps) {
        super(props);
        this.state = {
            data: {
                albums: [],
                artist: {
                    external_id: "",
                    id: 0,
                    name: "",
                }
            },
            isLoading: true,
        }
    }

    public async componentDidMount() {
        const ls = new LibraryService();
        const result = await ls.getAlbums(this.props.match.params.artist_id);
        this.setState({ data: result, isLoading: false });
    }

    public render() {
        if (this.state.isLoading) {
            return (
                <div>Loading...</div>
            )
        } else {
            return (
                <React.Fragment>
                    <div className="row no-gutters">
                        <div className="row">
                            <div className="col-sm-12 artist-header">
                                <h2><Link to="/artists">Artists</Link> <i className="fas fa-chevron-right chevron" /> {this.state.data.artist.name}</h2>
                                <div>
                                    <span>{this.state.data.albums.length} Albums</span>
                                </div>
                            </div>
                            <div className="row no-gutters">
                                {this.state.data.albums.map(album => (
                                    // tslint:disable-next-line:jsx-no-lambda
                                    <div className="col-xs-6 col-sm-3" key={album.id} onClick={(e) => this.onClick(e, album.id)} onMouseEnter={this.onHover} onMouseLeave={this.onOut} style={{ height: 200 + "px", backgroundSize: "cover", backgroundPosition: "center", backgroundImage: `url(http://localhost:90/images/${album.external_id}.jpg)` }}>
                                        <div className="album-card">
                                            <h3>{album.name}</h3>
                                            <h5>{album.year}</h5>
                                        </div>
                                    </div>
                                ))}
                            </div>
                        </div>
                    </div>
                </React.Fragment>

            );
        }

    }

    private onClick = (event: React.MouseEvent<HTMLElement>, albumId: number) => {
        this.props.history.push(`/album/${albumId}`);
    }

    private onHover = (event: React.MouseEvent<HTMLElement>) => {
        $(event.currentTarget).find('.album-card').removeClass('fadeOut').addClass('fadeIn');
        // $(event.currentTarget).addClass('grow').removeClass('shrink');
    };

    private onOut = (event: React.MouseEvent<HTMLElement>) => {
        $(event.currentTarget).find('.album-card').addClass('fadeOut').removeClass('fadeIn');
        // $(event.currentTarget).addClass('shrink').removeClass('grow');
    }
}