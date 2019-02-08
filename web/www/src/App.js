import React, { Component } from "react";
import * as $ from "jquery";
import "bootstrap/dist/js/bootstrap";
import "./App.css";
import * as axios from "axios";
import Sidebar from "./components/Sidebar";
import Topbar from "./components/Topbar";
import PageHeading from "./components/PageHeading";
import ArtistCard from "./components/ArtistCard";
import AlbumCard from "./components/AlbumCard";
import { BrowserRouter as Router, Route, Link, Switch } from "react-router-dom";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

class SongView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      artist: {},
      album: {},
      songs: []
    };
  }

  playSong = (song, album, artist) => {
    //e.preventDefault();
    this.props.playSongHandler(song, album, artist);
    toast.success(`${song.name} added to queue`);
  };

  addAllsongs = () => {
    const { artist, album, songs } = this.state;
    songs.forEach(song => {
      this.playSong(song, album, artist);
    });
  };

  componentDidMount() {
    axios
      .get(
        `http://localhost:8000/api/library/songs/${
          this.props.match.params.album_id
        }`
      )
      .then(response =>
        this.setState({
          artist: response.data.artist,
          album: response.data.album,
          songs: response.data.songs
        })
      );
  }

  render() {
    const { artist, album, songs } = this.state;
    let data = songs.map(song => (
      <li key={song.id}>
        <a href="#" onClick={this.playSong.bind(this, song, album, artist)}>
          {song.name}
        </a>
      </li>
    ));
    return (
      <React.Fragment>
        <PageHeading title={`${artist.name} - ${album.name}`}>
          <a
            href="#"
            class="d-none d-sm-inline-block btn btn-sm btn-primary shadow-sm"
            onClick={this.addAllsongs}
          >
            <i class="fas fa-trash fa-sm text-white-50" /> Add to Queue
          </a>
        </PageHeading>
        <div className="row">
          <ul>{data}</ul>
        </div>
      </React.Fragment>
    );
  }
}

class AlbumView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      artist: "",
      albums: []
    };
  }

  addAlbumToQueue = album => {
    axios
      .get(`http://localhost:8000/api/library/songs/${album.id}`)
      .then(response => {
        let artist = response.data.artist;
        let album = response.data.album;
        let songs = response.data.songs;
        songs.forEach(song => {
          this.props.queueAlbumHandler(song, album, artist);
        });
        toast.success(`${album.name} added to queue`);
      });
  };

  componentDidMount() {
    axios
      .get(
        `http://localhost:8000/api/library/albums/${
          this.props.match.params.artist_id
        }`
      )
      .then(response =>
        this.setState({
          albums: response.data.albums,
          artist: response.data.artist.name
        })
      );
  }

  render() {
    const { albums, artist } = this.state;
    let data = albums.map(album => (
      <AlbumCard
        key={album.id}
        album={album}
        history={this.props.history}
        addHandler={this.addAlbumToQueue}
      />
    ));
    return (
      <React.Fragment>
        <PageHeading title={artist} />
        <div className="row">{data}</div>
      </React.Fragment>
    );
  }
}

class ArtistView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      error: null,
      isLoaded: false,
      artists: []
    };
  }

  componentDidMount() {
    axios
      .get("http://localhost:8000/api/library/artists")
      .then(response =>
        this.setState({ isLoaded: true, artists: response.data.artists })
      );
  }

  render() {
    const { error, isLoaded, artists } = this.state;
    if (error) {
      return <div>Error: {error.message}</div>;
    } else if (!isLoaded) {
      return <div>Loading...</div>;
    } else {
      let data = artists.map(item => (
        <ArtistCard key={item.id} artist={item} history={this.props.history} />
      ));
      return (
        <React.Fragment>
          <PageHeading title="Music Library" />
          <div className="row">{data}</div>
        </React.Fragment>
      );
    }
  }
}

class QueueView extends Component {
  constructor(props) {
    super(props);
  }

  switchToTrack = num => {
    this.props.currentSongHandler(num);
  };

  render() {
    let items = this.props.queue.map((item, idx) => (
      <tr
        key={idx}
        className={this.props.queuePosition == idx ? "table-primary" : ""}
        onClick={this.switchToTrack.bind(this, idx)}
        style={{ cursor: "pointer" }}
      >
        <th scope="row">{idx + 1}</th>
        <td>{item.artist.name}</td>
        <td>{item.album.name}</td>
        <td>{item.song.name}</td>
      </tr>
    ));
    return (
      <React.Fragment>
        <PageHeading title="Queue">
          <a
            href="#"
            class="d-none d-sm-inline-block btn btn-sm btn-primary shadow-sm"
            onClick={this.props.clearQueueHandler}
          >
            <i class="fas fa-trash fa-sm text-white-50" /> Clear Queue
          </a>
        </PageHeading>
        <div className="row">
          <table className="table table-hover table-sm">
            <thead>
              <tr>
                <th scope="col">#</th>
                <th scope="col">Artist</th>
                <th scope="col">Album</th>
                <th scope="col">Song</th>
              </tr>
            </thead>
            <tbody>{items}</tbody>
          </table>
        </div>
      </React.Fragment>
    );
  }
}

class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      currentSong: null,
      album: null,
      artist: null,
      queue: [],
      queuePosition: 0
    };
  }

  componentDidMount() {
    let queue = JSON.parse(localStorage.getItem("queue")) || [];
    let position = localStorage.getItem("queuePosition") || 0;

    let song = null;
    if (queue.length > 0) {
      song = queue[position];
    }

    if (song == null) {
      return;
    }
    this.setState({
      currentSong: song.song,
      artist: song.artist,
      album: song.album,
      queue: queue,
      queuePosition: position
    });
  }

  playSong = (song, album, artist) => {
    let queue = this.state.queue;

    queue.push({ song: song, album: album, artist: artist });
    localStorage.setItem("queue", JSON.stringify(queue));
    if (queue.length == 1) {
      this.setState({
        currentSong: song,
        album: album,
        artist: artist,
        queue: queue
      });
    } else {
      this.setState({
        queue: queue
      });
    }
  };

  getNextSong = () => {
    let position = this.state.queuePosition;
    let queue = this.state.queue;

    if (position + 1 >= queue.length) {
      position = 0;
    } else {
      position++;
    }

    this.setState({
      queuePosition: position,
      currentSong: queue[position].song,
      album: queue[position].album,
      artist: queue[position].artist
    });
    localStorage.setItem("queuePosition", position);
  };

  getPrevSong = () => {
    let position = this.state.queuePosition;
    let queue = this.state.queue;

    if (position - 1 == -1) {
      position = queue.length - 1;
    } else {
      position--;
    }

    this.setState({
      queuePosition: position,
      currentSong: queue[position].song,
      album: queue[position].album,
      artist: queue[position].artist
    });
    localStorage.setItem("queuePosition", position);
  };

  setCurrentSong = songIdx => {
    let queue = this.state.queue;
    this.setState({
      queuePosition: songIdx,
      currentSong: queue[songIdx].song,
      album: queue[songIdx].album,
      artist: queue[songIdx].artist
    });
    localStorage.setItem("queuePosition", songIdx);
  };

  clearQueueHandler = () => {
    this.setState({
      queue: [],
      queuePosition: 0,
      currentSong: null,
      artist: null,
      album: null
    });
    localStorage.setItem("queue", null);
    localStorage.setItem("queuePosition", null);
  };

  render() {
    return (
      <div id="wrapper">
        <ToastContainer />

        <Sidebar
          currentSong={this.state.currentSong}
          album={this.state.album}
          artist={this.state.artist}
          nextSongHandler={this.getNextSong.bind(this)}
          prevSongHandler={this.getPrevSong.bind(this)}
        />
        <div id="content-wrapper" className="d-flex flex-column">
          <div id="content">
            <Topbar />
            <div className="container-fluid">
              <Switch>
                <Route
                  path="/album/:album_id"
                  render={routerProps => (
                    <SongView
                      {...routerProps}
                      playSongHandler={this.playSong.bind(this)}
                    />
                  )}
                />
                <Route
                  path="/artist/:artist_id"
                  render={routerProps => (
                    <AlbumView
                      {...routerProps}
                      queueAlbumHandler={this.playSong.bind(this)}
                    />
                  )}
                />
                <Route path="/library/artists" component={ArtistView} />
                <Route
                  path="/queue"
                  render={routerProps => (
                    <QueueView
                      {...routerProps}
                      queue={this.state.queue}
                      queuePosition={this.state.queuePosition}
                      clearQueueHandler={this.clearQueueHandler.bind(this)}
                      currentSongHandler={this.setCurrentSong.bind(this)}
                    />
                  )}
                />
              </Switch>
            </div>
          </div>
          <footer className="sticky-footer bg-white">
            <div className="container my-auto">
              <div className="copyright text-center my-auto">
                <span>Copyright &copy; Your Website 2019</span>
              </div>
            </div>
          </footer>
        </div>
      </div>
    );
  }
}

export default App;
