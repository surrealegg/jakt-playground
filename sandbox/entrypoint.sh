#!/bin/sh

run() {
    $@
    if [ $? -ne 0 ]; then
        exit 1
    fi
}

CODEGEN=${CODEGEN:-0};

run jakt input.jakt

if [ $CODEGEN -eq 1 ]; then
    cat output.cpp
    exit 0
fi

run clang++ -std=c++20 -I/usr/local/include/runtime -Wno-user-defined-literals /playground/output.cpp
/playground/a.out
rm -f /playground/a.out /playground/output.cpp