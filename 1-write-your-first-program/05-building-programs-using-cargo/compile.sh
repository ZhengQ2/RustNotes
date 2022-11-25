#!/bin/bash

if [ $1 = "-rustc" ]; then
    # compile a single rust file
    rustc $2
    exit 1
elif [ $1 = "-cargo" ]; then
    # compile a cargo project with debugging info and run it
    cargo run
    exit 1
elif [ $1 = "-cargo-final" ]; then
    # compile a cargo project with optimizations
    cargo build --release
    exit 1
fi
