import * as React from 'react';
import { Link } from 'react-router-dom';

export class MenuBar extends React.Component<{}, {}> {
    constructor(props: object) {
        super(props);
    }

    public render() {
        return (
            <div className="menu-bar">
                <div className="container-fluid">
                    <div className="row">
                        <div className="col-sm-3 App-title">
                            <h3><i className="fas fa-music" /> <span>Music Manager</span></h3>
                        </div>
                        <div className="col-sm-3">
                            <div className="btn-group">
                                <button className="btn dropdown-toggle" type="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                                    Menu
                                </button>
                                <div className="dropdown-menu">
                                    <Link to="/artists" className="dropdown-item">Artists</Link>
                                    <Link to="/albums" className="dropdown-item">Albums</Link>
                                    <Link to="/songs" className="dropdown-item">Songs</Link>
                                    <div className="dropdown-divider" />
                                    <Link to="/playlists" className="dropdown-item">Playlists</Link>
                                    <div className="dropdown-divider" />
                                    <Link to="/tasks" className="dropdown-item">Tasks</Link>
                                </div>
                            </div>
                        </div>
                        <div className="col-sm-6">
                            {this.props.children}
                        </div>
                    </div>
                </div>
            </div>
        )
    }
}