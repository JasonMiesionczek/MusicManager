import * as $ from 'jquery';
import * as React from 'react';
import { QueueItem } from '../App';

interface QueueViewProps {
    queue: QueueItem[];
    currentSongHandler: any;
}

export class QueueView extends React.Component<QueueViewProps, {}> {
    constructor(props: QueueViewProps) {
        super(props);
    }

    public render() {
        return (
            <React.Fragment>
                <div className="row">
                    <div className="col-sm-12 artist-header">
                        <h2>Queue</h2>
                    </div>
                </div>
                <div className="row no-gutters">
                    {this.props.queue.map((item, idx) => (
                        <div className="col-xs-12 col-sm-12" key={item.song.id} onClick={this.props.currentSongHandler.bind(this, idx)} onMouseEnter={this.onHover} onMouseLeave={this.onOut} style={{ height: 100 + "px", backgroundSize: "1024px 100px", backgroundRepeat: "no-repeat", backgroundPosition: "right", backgroundImage: `url(http://localhost:90/images/${item.song.filename}.png)` }}>
                            <div className="song-card">
                                <h5>{idx + 1}</h5>
                                <h3>{item.artist.name} / {item.album.name} / {item.song.name}</h3>
                            </div>
                        </div>
                    ))}
                </div>
            </React.Fragment>
        );
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