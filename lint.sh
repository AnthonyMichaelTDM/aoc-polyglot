# utility script that runs the linters of the various programming languages supported by the project
# usage: ./lint.sh

# Rust
if command -v cargo &> /dev/null
then
    echo "Running cargo clippy"
    cargo clippy --all-targets --all-features -- -D clippy::all -D clippy::pedantic -D clippy::nursery -D clippy::perf -D clippy::cargo -D warnings 
else
    echo "cargo not installed, skipping rust linter"
fi

# Zig
if command -v zig &> /dev/null
then
    echo "Running zig fmt"
    zig fmt --check --color on --ast-check .
else
    echo "zig not installed, skipping zig linter"
fi

# Python
if command -v mypy &> /dev/null
then
    echo "Running mypy"
    mypy .
else
    echo "mypy not installed, skipping python linter"
fi

# TODO: run other test suites here

