#!/bin/bash

CODEGEN=${CODEGEN:-0};

run() {
    timeout 5 $@
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        if [ $exit_code -eq 124 ]; then
            >&2 echo "Timeout while running '$@'"
        fi
        exit $exit_code
    fi
}

run jakt input.jakt

if [ $CODEGEN -eq 1 ]; then
    cat output.cpp
    exit 0
fi

run clang++ -std=c++20 -I/usr/local/include/runtime -Wno-user-defined-literals -fcolor-diagnostics /playground/output.cpp
run /playground/a.out
rm -f /playground/a.out /playground/output.cpp