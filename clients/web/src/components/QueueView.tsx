import * as $ from 'jquery';
import * as React from 'react';
import { QueueItem } from '../App';

interface QueueViewProps {
    queue: QueueItem[];
    currentSongHandler: any;
    clearQueueHandler: any;
    queuePosition: number;
}

export class QueueView extends React.Component<QueueViewProps, {}> {
    constructor(props: QueueViewProps) {
        super(props);
    }

    public render() {
        return (
            <React.Fragment>
                <div className="row">
                    <div className="col-sm-11 artist-header">
                        <h2>Queue</h2>
                    </div>
                    <div className="col-sm-1" style={{ textAlign: "center" }}>
                        <h3><i className="fas fa-trash" onClick={this.props.clearQueueHandler} /></h3>
                    </div>
                </div>
                <div className="row no-gutters">
                    {this.props.queue.map((item, idx) => (
                        <div className="col-xs-12 col-sm-12" key={idx} onClick={this.props.currentSongHandler.bind(this, idx)} onMouseEnter={this.onHover} onMouseLeave={this.onOut} style={{ height: 100 + "px", backgroundSize: "1024px 100px", backgroundRepeat: "no-repeat", backgroundPosition: "right", backgroundImage: `url(http://159.65.44.81:90/images/${item.song.filename}.png)` }}>
                            <div className={`song-card ${idx === this.props.queuePosition ? 'song-card-active' : ''}`}>
                                <h5>{idx + 1} <i className="fas fa-chevron-right chevron" /> {item.artist.name} <i className="fas fa-chevron-right chevron" /> {item.album.name}</h5>
                                <h3>{item.song.name}</h3>
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
