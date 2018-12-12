#!/bin/sh

if [ $# -ne 1 ]; then
    echo "Must have at exactly one parameter"
    exit
fi

day=$1
name="$(printf "day_%02d" "$day")"

# Create cargo binary project
cargo new $name

# Retrieve the day's input from the AOC site
session="$(cat session.txt)"
curl --header "Cookie: session="$session"" https://adventofcode.com/2018/day/$day/input > ./$name/input.txt
touch ./$name/sample_input.txt

# Let's go!
code ./$name/