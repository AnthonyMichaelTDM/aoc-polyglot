#!/bin/sh

# utility script that runs the python test suite
# usage: ./test.sh

# Python
if command -v pytest &> /dev/null
then
    echo "Running pytest"
    pytest .
else
    echo "pytest not installed, skipping python test suite"
fi
