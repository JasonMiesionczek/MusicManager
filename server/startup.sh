#!/bin/bash

Xvfb :99 -screen 0 1024x768x16 &
/app/cli db migrate
/app/task-runner
