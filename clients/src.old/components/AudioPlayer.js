import React, { Component } from "react";
import { BrowserRouter, Route, Link } from "react-router-dom";

const PlayPauseButton = props => {
  let icon = props.paused ? "play-circle" : "pause-circle";
  return (
    <button onClick={props.handler}>
      <i className={`fas fa-${icon}`} />
    </button>
  );
};

const PrevTrackButton = props => {
  return (
    <button onClick={props.handler}>
      <i className={`fas fa-chevron-left`} />
    </button>
  );
};

const NextTrackButton = props => {
  return (
    <button onClick={props.handler}>
      <i className={`fas fa-chevron-right`} />
    </button>
  );
};

const ProgressBar = props => {
  let percent = Math.ceil((props.current / props.duration) * 100);
  return (
    <div className="progress" style={{ height: "3px" }}>
      <div
        className="progress-bar bg-warning"
        role="progressbar"
        style={{ width: `${percent}%` }}
        aria-valuenow={percent}
        aria-valuemin="0"
        aria-valuemax="100"
      />
    </div>
  );
};

const convertTime = sec => {
  var hours = Math.floor(sec / 3600);
  hours >= 1 ? (sec = sec - hours * 3600) : (hours = "");
  var min = Math.floor(sec / 60);
  min >= 1 ? (sec = sec - min * 60) : (min = "0");
  sec < 1 ? (sec = "00") : void 0;

  min.toString().length == 1 ? (min = min) : void 0;
  sec.toString().length == 1 ? (sec = "0" + sec) : void 0;

  return hours + min + ":" + sec;
};

class AudioPlayer extends Component {
  constructor(props) {
    super(props);
    this.state = {
      song: props.song,
      album: null,
      artist: null,
      position: 0,
      duration: 0,
      paused: true
    };
  }

  onPlayHandler = () => {
    this.setState({ paused: false, duration: Math.ceil(this.audio.duration) });
  };

  onPauseHandler = () => {
    this.setState({ paused: true });
  };

  buttonHandler = () => {
    if (this.state.paused) {
      this.audio.play();
    } else {
      this.audio.pause();
    }
  };

  timeHandler = () => {
    let time = Math.ceil(this.audio.currentTime);
    this.setState({ position: time });
  };

  componentDidMount() {
    this.audio = document.querySelector("#audio_player");
  }

  render() {
    let songUrl = "";
    let albumUrl = null;
    if (this.props.song) {
      songUrl = `/music/${this.props.song.filename}`;
    }
    if (this.props.album) {
      albumUrl = `/images/${this.props.album.external_id}.jpg`;
    }
    let albumName = this.props.album ? this.props.album.name : "";
    return (
      <li className="nav-item active">
        <Link to="/queue" className="nav-link">
          <i className="fas fa-fw fa-tachometer-alt" />
          <span>Now Playing</span>
          <div>{this.props.artist ? this.props.artist.name : ""}</div>
          <div>{albumName}</div>
          <div>{this.props.song ? this.props.song.name : ""}</div>
          {albumUrl ? (
            <img src={albumUrl} width="100" height="100" alt={albumName} />
          ) : (
            ""
          )}
          <audio
            id="audio_player"
            src={songUrl}
            autoPlay
            onPlay={this.onPlayHandler}
            onPause={this.onPauseHandler}
            onTimeUpdate={this.timeHandler}
            onEnded={this.props.nextSongHandler}
          />
          <div>
            <PrevTrackButton handler={this.props.prevSongHandler} />
            <PlayPauseButton
              paused={this.state.paused}
              handler={this.buttonHandler}
            />
            <NextTrackButton handler={this.props.nextSongHandler} />
            <span>
              {convertTime(this.state.position)} /{" "}
              {convertTime(this.state.duration)}
            </span>
          </div>
          <div>
            <ProgressBar
              current={this.state.position}
              duration={this.state.duration}
            />
          </div>
        </Link>
      </li>
    );
  }
}

export default AudioPlayer;
