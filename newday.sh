#!/bin/bash
# Usage ./newday.sh <DAY>
DAY=$1

set -e

source .env

mkdir -p inputs puzzles
aoc download --input-file inputs/day$DAY.txt --puzzle-file puzzles/day$DAY.md --overwrite
cp template.rs src/bin/day$DAY.rs

code puzzles/day$DAY.md src/bin/day$DAY.rs inputs/day$DAY.txt