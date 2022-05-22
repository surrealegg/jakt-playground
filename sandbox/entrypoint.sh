#!/bin/sh

run() {
    echo $@
    $@
    if [ $? -ne 0 ]; then
        exit 1
    fi
}

announce() {
    printf "$1\n\n"
}

announce Compiling:
run jakt input.jakt
run clang++ -std=c++20 -I/usr/local/include/runtime -Wno-user-defined-literals /playground/output.cpp
printf "\n"

announce Executing:
/playground/a.out
rm -f /playground/a.out /playground/output.cpp