# utility script that runs code formatters for the various programming languages supported by the project
# usage: ./fmt.sh

# Rust
if command -v cargo &> /dev/null
then
    echo "Running cargo fmt"
    cargo fmt --all
else
    echo "cargo not installed, skipping rust formatter"
fi

# Zig
if command -v zig &> /dev/null
then
    echo "Running zig fmt"
    zig fmt --color on .
else
    echo "zig not installed, skipping zig formatter"
fi

# TODO: run other code formatters here