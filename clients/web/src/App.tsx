import * as React from "react";
import "./App.css";

import { Route, Switch } from 'react-router';
import { AlbumView } from './components/AlbumView';
import ArtistView from './components/ArtistView';
import { AudioPlayer } from './components/AudioPlayer';
import { MenuBar } from './components/MenuBar';
import { QueueView } from './components/QueueView';
import { SongView } from './components/SongView';
import { Album, Artist, Song } from './services/LibraryService';

interface AppState {
  currentSong: Song | null;
  album: Album | null;
  artist: Artist | null;
  queue: QueueItem[];
  queuePosition: number;
}

export interface QueueItem {
  song: Song;
  album: Album;
  artist: Artist;
}

class App extends React.Component<{}, AppState> {
  constructor(props: object) {
    super(props);
    this.state = {
      album: null,
      artist: null,
      currentSong: null,
      queue: [],
      queuePosition: 0
    }
  }

  public componentDidMount() {

    const queueData = localStorage.getItem("queue");
    if (queueData === null) {
      return;
    }
    const queue = JSON.parse(queueData) || [];

    this.setState({
      queue
    });
  }

  public render() {
    return (
      <div>
        <MenuBar>
          <AudioPlayer queue={this.state.queue} song={this.state.currentSong} album={this.state.album} artist={this.state.artist} nextSongHandler={this.nextSongHandler} prevSongHandler={this.prevSongHandler} />
        </MenuBar>
        <Switch>
          <Route exact={true} path="/" component={ArtistView} />
          <Route path="/artists" component={ArtistView} />
          <Route path="/artist/:artist_id" component={AlbumView} />
          <Route path="/album/:album_id" render={routerProps => (<SongView {...routerProps} playSongHandler={this.playSong.bind(this)} />)} />
          <Route path="/queue" render={routerProps => (<QueueView {...routerProps} queue={this.state.queue} currentSongHandler={this.setCurrentSong.bind(this)} />)} />
        </Switch>
      </div>
    );
  }

  private playSong = (song: Song, album: Album, artist: Artist) => {
    const queue = this.state.queue;
    queue.push({ song, album, artist });
    localStorage.setItem("queue", JSON.stringify(queue));
    if (queue.length === 1) {
      this.setState({ currentSong: song, album, artist, queue });
    } else {
      this.setState({ queue });
    }
  }

  private nextSongHandler = (song: Song, album: Album, artist: Artist) => {
    let position = this.state.queuePosition;
    const queue = this.state.queue;

    if (position + 1 >= queue.length) {
      position = 0;
    } else {
      position++;
    }

    this.setState({
      album: queue[position].album,
      artist: queue[position].artist,
      currentSong: queue[position].song,
      queuePosition: position,
    });
    localStorage.setItem("queuePosition", position.toString());
  }

  private prevSongHandler = (song: Song, album: Album, artist: Artist) => {
    let position = this.state.queuePosition;
    const queue = this.state.queue;

    if (position - 1 === -1) {
      position = queue.length - 1;
    } else {
      position--;
    }

    this.setState({
      album: queue[position].album,
      artist: queue[position].artist,
      currentSong: queue[position].song,
      queuePosition: position,
    });
    localStorage.setItem("queuePosition", position.toString());
  }

  private setCurrentSong = (idx: number) => {
    const queue = this.state.queue;
    this.setState({
      album: queue[idx].album,
      artist: queue[idx].artist,
      currentSong: queue[idx].song,
      queuePosition: idx,
    });
  }
}

export default App;
