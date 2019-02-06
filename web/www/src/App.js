import React, { Component } from "react";
import * as $ from "jquery";
import "bootstrap/dist/js/bootstrap";
import "./App.css";
import * as axios from "axios";
import Sidebar from "./components/Sidebar";
import Topbar from "./components/Topbar";
import PageHeading from "./components/PageHeading";
import { BrowserRouter as Router, Route, Link, Switch } from "react-router-dom";

class ArtistCard extends React.Component {
  state = {
    isOpen: false
  };

  toggleOpen = () => this.setState({ isOpen: !this.state.isOpen });
  handleClick = e => {
    e.preventDefault();
    console.log(this.props);
    console.log(this.context);
    this.props.history.push(`/artist/${this.props.artist.id}`);
  };

  render() {
    const menuClass = `dropdown-menu${this.state.isOpen ? " show" : ""}`;
    return (
      <div
        className="col-xl-3 col-lg-2 clickable-card"
        onClick={this.handleClick}
      >
        <div className="card shadow mb-4">
          <div className="card-header py-3 d-flex flex-row align-items-center justify-content-between">
            <h6 className="m-0 font-weight-bold text-primary">
              {this.props.artist.name}
            </h6>
            <div className="dropdown no-arrow" onClick={this.toggleOpen}>
              <a
                className="dropdown-toggle"
                href="#"
                role="button"
                id="dropdownMenuLink"
                data-toggle="dropdown"
                aria-haspopup="true"
                aria-expanded="false"
              >
                <i className="fas fa-ellipsis-v fa-sm fa-fw text-gray-400" />
              </a>
              <div
                className={`${menuClass} dropdown-menu dropdown-menu-right shadow animated--fade-in`}
                aria-labelledby="dropdownMenuLink"
              >
                <div className="dropdown-header">Dropdown Header:</div>
                <a className="dropdown-item" href="#">
                  Action
                </a>
                <a className="dropdown-item" href="#">
                  Another action
                </a>
                <div className="dropdown-divider" />
                <a className="dropdown-item" href="#">
                  Something else here
                </a>
              </div>
            </div>
          </div>
          <div
            className="card-body"
            style={{
              height: 200 + "px",
              backgroundSize: "cover",
              backgroundPosition: "center",
              backgroundImage: `url(http://localhost:8000/images/${
                this.props.artist.external_id
              }.jpg)`
            }}
          />
        </div>
      </div>
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

  componentDidMount() {
    axios
      .get(
        `http://localhost:8000/api/library/albums/${
          this.props.match.params.artist_id
        }`
      )
      .then(response => this.setState({ albums: response.data }));
  }

  render() {
    const albums = this.state.albums;
    let data = albums.map(album => <li key={album.id}>{album.name}</li>);
    return <ul>{data}</ul>;
  }
}

class Library extends Component {
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

  /*
<li key={item.name}>
          {item.name}
          <img
            src={`http://localhost:8000/images/${item.external_id}.jpg`}
            alt=""
          />
        </li>
  */

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

// function Welcome(props) {
//   let response =
//   return (
//     <ul>
//       {response.data.artists.map(artist => (
//         <li>{artist.name}</li>
//       ))}
//     </ul>
//   );
// }

class App extends Component {
  render() {
    return (
      <div id="wrapper">
        <Sidebar />
        <div id="content-wrapper" className="d-flex flex-column">
          <div id="content">
            <Topbar />
            <div className="container-fluid">
              <Switch>
                <Route path="/artist/:artist_id" component={AlbumView} />
                <Route path="/library" component={Library} />
              </Switch>
            </div>
          </div>
        </div>
      </div>
    );
  }
}

export default App;
