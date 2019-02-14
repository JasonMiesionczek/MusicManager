#!/bin/bash
sudo Xvfb :99 -screen 0 1024x768x16 &
cargo run -p cli -- db migrate && cargo watch -x 'run --bin task-runner'