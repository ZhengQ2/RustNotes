#!/bin/bash

if [ $1 = "--help" ]; then
    echo "Usage: compile.sh [options] [FILE]"
    echo "Options:"
    echo "  --help        Display this help message"
    echo "  --compile     Compiled the program"
    echo "  --cargo       Run cargo"
    echo "  --cargo-final Run cargo with released version"
    exit 0
fi
elif [ $1 = "--compile" ]; then
    # compile a single rust file
    rustc $2
    exit 0
elif [ $1 = "--cargo" ]; then
    # compile a cargo project with debugging info and run it
    cargo run
    exit 0
elif [ $1 = "--cargo-final" ]; then
    # compile a cargo project with optimizations
    cargo build --release
    exit 0
fi
