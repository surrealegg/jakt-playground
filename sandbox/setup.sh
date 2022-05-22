#!/bin/sh

docker image prune
docker build -t 'jakt_sandbox' ./sandbox
docker images