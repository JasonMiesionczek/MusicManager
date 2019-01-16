use web_view::*;

use std::{env, thread, time::Duration};

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Debug)]
pub enum Mode {
    Album,
    Song,
}

#[derive(Debug)]
pub struct Config {
    mode: Mode,
    artist: Option<String>,
    album_id: Option<String>,
}

impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let mode = match args.next() {
            Some(m) => match m.as_ref() {
                "album" => Mode::Album,
                "song" => Mode::Song,
                _ => return Err("unknown mode"),
            },
            None => return Err("no mode specified"),
        };

        let artist = match mode {
            Mode::Album => match args.next() {
                Some(a) => Some(a),
                None => return Err("no artist specified"),
            },
            Mode::Song => None,
        };

        let album_id = match mode {
            Mode::Album => None,
            Mode::Song => match args.next() {
                Some(s) => Some(s),
                None => return Err("no album id specified"),
            },
        };

        Ok(Config {
            mode,
            artist,
            album_id,
        })
    }
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Albums { data: Vec<Album> },
    Songs { data: Vec<Song> },
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Album {
    id: String,
    name: String,
    artist: String,
    image: String,
    year: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Song {
    id: String,
    name: String,
    num: u32,
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    let album_script = r#"
	var bandName = "";
        function getAlbums() {
            var data = [];
            bandName = document.querySelector('[role="heading"]').text.runs[0].text;

            var showAllAlbumsLink = document.querySelector('[title="See all"]');
            if (showAllAlbumsLink == undefined) {
                var albums = document.querySelector('ytmusic-carousel-shelf-renderer').data.contents;
                for (var i = 0; i < albums.length; i++) {
                    var id = albums[i].musicTwoRowItemRenderer.doubleTapNavigationEndpoint.watchPlaylistEndpoint.playlistId;
                    var albumName = albums[i].musicTwoRowItemRenderer.title.runs[0].text;
                    data.push({id: id, name: albumName, artist: bandName});
                }
                external.invoke(JSON.stringify({cmd: 'albums', data: data}));
            } else {
		clickAlbumLink();
            }
        }

	function clickAlbumLink() {
		document.querySelector('[title="See all"]').click();
		setTimeout(function () {
		    var data = [];
		    var albums = document.querySelector('ytmusic-section-list-renderer').data.contents[0].musicShelfRenderer.contents;
		    for (var i = 0; i < albums.length; i++) {
			    var id = albums[i].musicResponsiveListItemRenderer.menu.menuRenderer.items[3].menuServiceItemRenderer.serviceEndpoint.queueAddEndpoint.queueTarget.playlistId;
			    var albumName = albums[i].musicResponsiveListItemRenderer.flexColumns[0].musicResponsiveListItemFlexColumnRenderer.text.runs[0].text;
                var imageUrl = document.querySelectorAll('ytmusic-responsive-list-item-renderer')[i].querySelector('img').src;
                var year = document.querySelectorAll('ytmusic-responsive-list-item-renderer')[i].querySelector('.secondary-flex-columns').querySelector('yt-formatted-string').text.runs[2].text;
			    data.push(
                    {
                        id: id,
                        name: albumName,
                        artist: bandName,
                        image: imageUrl,
                        year: year
                    }
                );
		    }
		    external.invoke(JSON.stringify({cmd: 'albums', data: data}));
		}, 5000);
	}

        function start() {
            document.querySelector('a[href*="channel"]').click();
            setTimeout(getAlbums, 5000);
        }

        start();
    "#;

    let song_script = r#"
        function getSongData() {
            var data = [];
            var songs = document.querySelector('ytmusic-data-bound-album-release-tracks-shelf-renderer').data.shelfMold.musicShelfRenderer.contents;
            for (var i = 0; i < songs.length; i++) {
                var songId = songs[i].musicListItemRenderer.overlay.musicItemThumbnailOverlayRenderer.content.musicPlayButtonRenderer.playNavigationEndpoint.watchEndpoint.videoId;
                var albumId = songs[i].musicListItemRenderer.overlay.musicItemThumbnailOverlayRenderer.content.musicPlayButtonRenderer.playNavigationEndpoint.watchEndpoint.playlistId;
                var name = songs[i].musicListItemRenderer.overlay.musicItemThumbnailOverlayRenderer.content.musicPlayButtonRenderer.accessibilityPlayData.accessibilityData.label;
                name = name.substring(5);
                if (typeof songId == 'undefined' || songId == 'undefined') {
                    continue;
                }
                data.push({id: songId, name: name, num: i+1});
            }
            external.invoke(JSON.stringify({cmd: 'songs', data: data}));
        }

        function start() {
            setTimeout(getSongData, 5000);
        }

        start();
    "#;

    let script = match config.mode {
        Mode::Album => album_script,
        Mode::Song => song_script,
    };

    let url = match config.mode {
        Mode::Album => format!(
            "https://music.youtube.com/search?q={}",
            config.artist.unwrap()
        ),
        Mode::Song => format!(
            "https://music.youtube.com/playlist?list={}",
            config.album_id.unwrap()
        ),
    };

    let web_view = web_view::builder()
        .title("test")
        .content(Content::Url(url))
        .size(800, 600)
        .debug(false)
        .user_data(0)
        .invoke_handler(|webview, arg| {
            match serde_json::from_str(arg).unwrap() {
                Cmd::Albums { data } => {
                    if let Ok(json_str) = serde_json::to_string(&data) {
                        println!("{}", json_str);
                        webview.terminate();
                    }
                }
                Cmd::Songs { data } => {
                    if let Ok(json_str) = serde_json::to_string(&data) {
                        println!("{}", json_str);
                        webview.terminate();
                    }
                }
            }
            Ok(())
        })
        .build()
        .unwrap();

    let handle = web_view.handle();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        handle
            .dispatch(move |webview| webview.eval(script))
            .unwrap();
    });

    web_view.run().unwrap();
}
