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
    Image,
    Artist,
}

#[derive(Debug)]
pub struct Config {
    script: String,
    url: String,
}

impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let mode = match args.next() {
            Some(m) => match m.as_ref() {
                "album" => Mode::Album,
                "song" => Mode::Song,
                "image" => Mode::Image,
                "artist" => Mode::Artist,
                _ => return Err("unknown mode"),
            },
            None => return Err("no mode specified"),
        };

        let artist = match mode {
            Mode::Album | Mode::Artist => match args.next() {
                Some(a) => Some(a),
                None => return Err("no artist specified"),
            },
            Mode::Song | Mode::Image => None,
        };

        let album_id = match mode {
            Mode::Album | Mode::Artist => None,
            Mode::Song | Mode::Image => match args.next() {
                Some(s) => Some(s),
                None => return Err("no album id specified"),
            },
        };

        let script = match mode {
            Mode::Album => include_str!("scripts/album.js").to_string(),
            Mode::Song => include_str!("scripts/songs.js")
                .to_string()
                .replace("%ALBUM_ID%", album_id.clone().unwrap().as_str()),
            Mode::Image => include_str!("scripts/album_image.js").to_string(),
            Mode::Artist => include_str!("scripts/artist_image.js").to_string(),
        };

        let url = match mode {
            Mode::Album | Mode::Artist => {
                format!("https://music.youtube.com/search?q={}", artist.unwrap())
            }
            Mode::Song | Mode::Image => format!(
                "https://music.youtube.com/playlist?list={}",
                album_id.unwrap()
            ),
        };

        Ok(Config { script, url })
    }
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Albums { data: Vec<Album> },
    Songs { data: Vec<Song> },
    Image { data: String },
    Artist { data: String },
    Abort,
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
    image: String,
    album_id: String,
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let web_view = web_view::builder()
        .title("test")
        .content(Content::Url(config.url))
        .size(1224, 768)
        .debug(true)
        .user_data(0)
        .invoke_handler(|webview, arg| {
            match serde_json::from_str(arg).unwrap() {
                Cmd::Abort => {
                    println!("ABORT");
                    webview.terminate();
                }
                Cmd::Artist { data } => {
                    println!("{}", data);
                    webview.terminate();
                }
                Cmd::Image { data } => {
                    println!("{}", data);
                    webview.terminate();
                }
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
    let script = config.script;
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        handle
            .dispatch(move |webview| webview.eval(script.as_str()))
            .unwrap();
    });

    web_view.run().unwrap();
}
