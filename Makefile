COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
SERVER_DIR = server

.PHONY: all bench build build-scraper check clean doc install scraper publish run test update run-api

all: build

migrate:
	cd $(SERVER_DIR) && cargo run -p cli -- db migrate

bench:
	@$(CARGO) bench

build:
	@$(CARGO) build

build-scraper:
	@$(CARGO) build -p scraper --release

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) doc

install: build
	@$(CARGO) install

scraper: build-scraper
	@$(CARGO) install --force --path tools/scraper

publish:
	@$(CARGO) publish

run: build
	@$(CARGO) run

run-api: build
	@$(CARGO) run -p web

test: build
	@$(CARGO) test

update:
	@$(CARGO) update
