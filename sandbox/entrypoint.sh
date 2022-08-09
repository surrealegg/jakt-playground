#!/bin/bash

run() {
    timeout 5 $@
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        if [ $exit_code -eq 124 ]; then
            >&2 echo "'$@' took way too long to run."
        fi
        exit $exit_code
    fi
}

run clang++ -std=c++20 -I/usr/local/include/runtime \
    -include-pch /usr/local/include/runtime/lib.h.gch \
    -Wno-user-defined-literals -no-pie -O0 \
    -fno-exceptions -fcolor-diagnostics /playground/input.cpp
run /playground/a.out
rm -f /playground/a.out