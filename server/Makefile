
.PHONY: all
all: help

.PHONY: help
help:
	@echo "wtf are you doing??"	

.PHONY: build-release
build-release:
	cargo build --release

.PHONY: build-debug
build-debug:
	cargo build

.PHONY: build-taskrunner-image-debug
build-taskrunner-image-debug: build-debug
	docker build -t taskrunner_image -f ./images/taskrunner/Dockerfile.debug .

.PHONY: build-taskrunner-image-release
build-taskrunner-image-release: build-release
	docker build -t taskrunner_image -f ./images/taskrunner/Dockerfile.release .

.PHONY: push-taskrunner-latest-debug
push-taskrunner-latest-debug: build-taskrunner-image-debug
	docker tag taskrunner_image musicmanager/taskrunner:latest-debug
	docker push musicmanager/taskrunner:latest-debug

.PHONY: push-taskrunner-latest-release
push-taskrunner-latest-release: build-taskrunner-image-release
	docker tag taskrunner_image musicmanager/taskrunner:latest-release
	docker push musicmanager/taskrunner:latest-release