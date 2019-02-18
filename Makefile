COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
SERVER_DIR = server
CONF_DIR = conf/systemd

.PHONY: all build install migrate 

all: build

migrate:
	cd $(SERVER_DIR) && cargo run -p cli -- db migrate

install:
	cd $(CONF_DIR) && cp *.service /etc/systemd/system
	systemctl enable xvfb.service
	systemctl enable webclient.service
	systemctl enable taskrunner.service
	systemctl enable apiserver.service
	systemctl enable staticfiles.service
