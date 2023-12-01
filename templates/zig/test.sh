#!/bin/sh

# utility script that runs the zig test suite
# usage: ./test.sh

# Zig
if command -v zig &> /dev/null
then
    echo "Running zig test"
    zig test --color on .
else
    echo "zig not installed, skipping zig test suite"
fi
