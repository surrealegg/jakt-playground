#!/bin/bash

if [[ ! -v JAKT_HOME ]]; then
    echo "JAKT_HOME is not set"
    exit 1
fi

cp -R $JAKT_HOME/runtime ./sandbox
docker build -t 'jakt_sandbox' ./sandbox
docker images