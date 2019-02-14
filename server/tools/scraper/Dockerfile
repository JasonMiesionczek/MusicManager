FROM rustlang/rust:nightly

RUN apt-get update && apt-get install -y libwebkit2gtk-4.0-dev

RUN USER=root chown -R 1000:1000 /usr/local/cargo

USER 1000

WORKDIR /usr/src/app

VOLUME ["/usr/local/cargo"]