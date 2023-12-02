#!/bin/sh

# utility script that runs the rust test suite
# usage: ./test.sh

# Rust
if command -v cargo &> /dev/null
then
    echo "Running cargo test"
    cargo test --quiet
else
    echo "cargo not installed, skipping rust test suite"
fi
