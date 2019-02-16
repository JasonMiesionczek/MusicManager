import "@/animations.css";
import { LibraryService } from '@/services/LibraryService';
import { History } from 'history';
import * as $ from 'jquery';
import { Component } from 'react';
import * as React from 'react';
import { ArtistsResult } from '../services/LibraryService';
import "./ArtistView.css";

interface ArtistViewProps {
    history: History;
}

class ArtistView extends Component<ArtistViewProps, { data: ArtistsResult, isLoading: boolean }> {
    constructor(props: ArtistViewProps) {
        super(props);
        this.state = { data: { artists: [] }, isLoading: true };
    }

    public async componentDidMount() {
        const ls = new LibraryService();
        const result = await ls.getArtists();
        this.setState({ data: result, isLoading: false });
    }

    public render() {
        if (this.state.isLoading) {
            return (<div>Loading...</div>)
        } else {
            return (
                <div className="row no-gutters" >
                    {this.state.data.artists.map(data => (
                        // tslint:disable-next-line:jsx-no-lambda
                        <div className="col-xs-6 col-sm-3" key={data.artist.id} onClick={(e) => this.onClick(e, data.artist.id)} onMouseEnter={this.onHover} onMouseLeave={this.onOut} style={{ height: 200 + "px", backgroundSize: "cover", backgroundPosition: "center", backgroundImage: `url(http://159.65.44.81:90/images/${data.artist.external_id}.jpg)` }}>
                            <div className="artist-card" >
                                <h3>{data.artist.name}</h3>
                                <div>
                                    <span>{data.album_count} Albums</span>
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            );
        }
    }

    private onClick = (event: React.MouseEvent<HTMLElement>, artistId: number) => {
        this.props.history.push(`/artist/${artistId}`);
    }

    private onHover = (event: React.MouseEvent<HTMLElement>) => {
        $(event.currentTarget).find('.artist-card').removeClass('fadeOut').addClass('fadeIn');
        // $(event.currentTarget).addClass('grow').removeClass('shrink');
    };

    private onOut = (event: React.MouseEvent<HTMLElement>) => {
        $(event.currentTarget).find('.artist-card').addClass('fadeOut').removeClass('fadeIn');
        // $(event.currentTarget).addClass('shrink').removeClass('grow');
    }
}

export default ArtistView;
