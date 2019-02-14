import * as React from 'react';
import { Link } from 'react-router-dom';
import { QueueItem } from '../App';
import { Album, Artist, Song } from '../services/LibraryService';

interface AudioPlayerState {
    song: Song | null;
    album: Album | null;
    artist: Artist | null;
    position: number;
    duration: number;
    paused: boolean;
}

interface AudioPlayerProps {
    song: Song | null;
    album: Album | null;
    artist: Artist | null;
    nextSongHandler: any;
    prevSongHandler: any;
    queue: QueueItem[];
}

const ProgressBar = (props: any) => {
    const percent = Math.ceil((props.current / props.duration) * 100);
    return (
        <div className="progress" style={{ height: "3px" }}>
            <div
                className="progress-bar bg-primary"
                role="progressbar"
                style={{ width: `${percent}%` }}
                aria-valuenow={percent}
                aria-valuemin={0}
                aria-valuemax={100}
            />
        </div>
    );
};

const PlayPauseButton = (props: any) => {
    const icon = props.paused ? "play-circle" : "pause-circle";
    return (
        <button onClick={props.handler}>
            <i className={`fas fa-${icon}`} />
        </button>
    );
};

const PrevTrackButton = (props: any) => {
    return (
        <button onClick={props.handler}>
            <i className={`fas fa-chevron-left`} />
        </button>
    );
};

const NextTrackButton = (props: any) => {
    return (
        <button onClick={props.handler}>
            <i className={`fas fa-chevron-right`} />
        </button>
    );
};

export class AudioPlayer extends React.Component<AudioPlayerProps, AudioPlayerState> {
    private audio: any;
    constructor(props: AudioPlayerProps) {
        super(props);
        this.state = {
            album: props.album,
            artist: props.artist,
            duration: 0,
            paused: true,
            position: 0,
            song: props.song,
        }
    }

    public componentWillUpdate() {
        this.audio = document.querySelector("#audio_player");
    }

    public render() {
        let songUrl = "";
        let songName = "<Select a song>";
        let artistName = "<Select a song>";
        let albumName = "<Select a song>";
        const { song, artist, album, queue } = this.props;

        if (song != null) {
            songUrl = `http://localhost:90/music/${song.filename}`;
            songName = song.name;
        }

        if (artist != null) {
            artistName = artist.name;
        }

        if (album != null) {
            albumName = album.name;
        }

        return (
            <>
                <div className="container">
                    <div className="row">
                        <div className="col-md-3 audio-player-artist">
                            <div style={{ fontWeight: "bold" }}>{artistName}</div>
                            <div>{albumName}</div>
                        </div>

                        <div className="col-md-6 audio-player-song">
                            {songName}
                            <div className="audio-player-time">
                                {this.convertTime(this.state.position)} /{" "}
                                {this.convertTime(this.state.duration)}
                            </div>
                            <ProgressBar current={this.state.position} duration={this.state.duration} />
                        </div>
                        <div className="col-md-2 audio-player-controls">
                            <PrevTrackButton handler={this.props.prevSongHandler} />
                            <PlayPauseButton paused={this.state.paused} handler={this.buttonHandler} />
                            <NextTrackButton handler={this.props.nextSongHandler} />
                        </div>
                        <div className="col-md-1 queue-icon">
                            <Link to="/queue">
                                <i data-count={queue.length} className={`fas fa-list-ol icon-grey ${queue.length > 0 ? 'badge' : ''}`}>&nbsp;</i>
                            </Link>
                        </div>
                    </div>
                </div>

                <audio
                    id="audio_player"
                    src={songUrl}
                    autoPlay={true}
                    onPlay={this.onPlayHandler}
                    onPause={this.onPauseHandler}
                    onTimeUpdate={this.timeHandler}
                    onEnded={this.props.nextSongHandler}
                />
            </>
        );


    }

    private buttonHandler = () => {
        if (this.state.paused) {
            this.audio.play();
        } else {
            this.audio.pause();
        }
    };

    private convertTime = (sec: any) => {
        const hours = Math.floor(sec / 3600);
        let hoursStr = "";
        hours >= 1 ? (sec = sec - hours * 3600) : (hoursStr = "");
        const min = Math.floor(sec / 60);
        let minStr = "";
        min >= 1 ? (sec = sec - min * 60) : (minStr = "0");
        sec < 1 ? (sec = "00") : (sec = sec);

        // min.toString().length === 1 ? (minStr = min.toString()) : 0;
        minStr = min.toString();
        sec.toString().length === 1 ? (sec = "0" + sec) : (sec = sec);

        return hoursStr + minStr + ":" + sec;
    };

    private onPlayHandler = () => {
        this.audio = document.querySelector("#audio_player");
        this.setState({ paused: false, duration: Math.ceil(this.audio.duration) });
    }

    private onPauseHandler = () => {
        this.setState({ paused: true });
    }

    private timeHandler = () => {
        const time = Math.ceil(this.audio.currentTime);
        this.setState({ position: time });
    }

}