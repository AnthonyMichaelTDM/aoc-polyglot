#!/bin/sh

# utility script that runs the test suites of the various programming languages supported by the project
# for every solution
# usage: ./test.sh

CHALLENGE_DIR=./challenges

# Iterate over year directories
for year_dir in $CHALLENGE_DIR/*; do
    case $year_dir in 
        $CHALLENGE_DIR/[0-9][0-9][0-9][0-9])
            ;;
        *)
            echo "Skipping $year_dir"
            continue # skip non-year directories
            ;;
    esac

    # Iterate over day directories
    for day_dir in $year_dir/*; do
        case $day_dir in 
            $year_dir/day[0-9][0-9])
                ;;
            *)
                echo "Skipping $day_dir"
                continue # skip non-day directories
                ;;
        esac

        # Iterate over programming language directories
        for lang_dir in $day_dir/*; do
            if [ ! -d "$lang_dir" ]; then
                continue # skip non-directories
            fi
            if [ ! -f "$lang_dir/test.sh" ]; then
                continue # skip directories without test.sh
            fi

            # Run tests for each language
            cd $lang_dir
            if [ -f "test.sh" ]; then
                echo "Running test.sh for $lang_dir"
                $SHELL test.sh
            fi
            cd ../../../../
        done # end for
    done # end for

done