import React, { Component } from "react";

class PageHeading extends Component {
  constructor(props) {
    super(props);
  }

  render() {
    return (
      <div className="d-sm-flex align-items-center justify-content-between mb-4">
        <h1 className="h3 mb-0 text-gray-800">{this.props.title}</h1>
        {this.props.children}
      </div>
    );
  }
}

export default PageHeading;
