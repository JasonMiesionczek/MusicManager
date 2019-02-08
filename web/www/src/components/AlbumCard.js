import React, { Component } from "react";

class AlbumCard extends React.Component {
  state = {
    isOpen: false
  };

  toggleOpen = () => this.setState({ isOpen: !this.state.isOpen });
  handleClick = e => {
    e.preventDefault();
    console.log(this.props);
    console.log(this.context);
    this.props.history.push(`/album/${this.props.album.id}`);
  };

  render() {
    const menuClass = `dropdown-menu${this.state.isOpen ? " show" : ""}`;
    return (
      <div className="col-xl-3 col-lg-2 clickable-card">
        <div className="card shadow mb-4">
          <div className="card-header py-3 d-flex flex-row align-items-center justify-content-between">
            <h6 className="m-0 font-weight-bold text-primary">
              <button onClick={this.handleClick}>
                {this.props.album.name} ({this.props.album.year})
              </button>
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
                <button
                  className="dropdown-item"
                  onClick={this.props.addHandler.bind(this, this.props.album)}
                >
                  Add to Queue
                </button>
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
                this.props.album.external_id
              }.jpg)`
            }}
          />
        </div>
      </div>
    );
  }
}
export default AlbumCard;
