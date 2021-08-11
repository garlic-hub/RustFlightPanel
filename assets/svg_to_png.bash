#!/usr/bin/env bash

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 <width> <height>"
    exit 1
fi

for f in *.svg; do
    out=$(basename "$f" .svg)
    inkscape -w "$1" -h "$2" "$f" -o "$out.png"
done
