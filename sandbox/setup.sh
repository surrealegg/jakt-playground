#!/bin/sh

if [ $(id -u) -ne 0 ]; then 
    echo "Root needed to run this script.";
    exit 1;
fi

docker build -t 'jakt_sandbox' ./sandbox
docker images