#!/bin/bash

if [[ ! -v JAKT_HOME ]]; then
    echo "JAKT_HOME is not set"
    exit 1
fi

if [[ ! -v JAKT_PLAYGROUND_HOME ]]; then
    echo "JAKT_PLAYGROUND_HOME is not set"
    exit 1
fi

cp -R $JAKT_HOME/runtime $JAKT_PLAYGROUND_HOME/sandbox/runtime
docker build -t 'jakt_sandbox' $JAKT_PLAYGROUND_HOME/sandbox
docker images