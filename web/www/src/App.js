import React, { Component } from "react";
import "./App.css";
import * as axios from "axios";

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
      .get("http://localhost:8000/api/library")
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
        <li key={item.name}>
          {item.name}
          <img
            src={`http://localhost:8000/images/${item.external_id}.jpg`}
            alt=""
          />
        </li>
      ));
      return <ul>{data}</ul>;
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
      <div className="App">
        <Library />
      </div>
    );
  }
}

export default App;
