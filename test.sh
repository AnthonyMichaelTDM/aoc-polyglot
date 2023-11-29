# utility script that runs the test suites of the various programming languages supported by the project
# usage: ./test.sh

# Rust
if command -v cargo &> /dev/null
then
    echo "Running cargo test"
    cargo test
else
    echo "cargo not installed, skipping rust test suite"
fi

# Zig
if command -v zig &> /dev/null
then
    echo "Running zig test"
    zig test --color on .
else
    echo "zig not installed, skipping zig test suite"
fi

# Python
if command -v pytest &> /dev/null
then
    echo "Running pytest"
    pytest .
else
    echo "pytest not installed, skipping python test suite"
fi

# TODO: run other test suites here