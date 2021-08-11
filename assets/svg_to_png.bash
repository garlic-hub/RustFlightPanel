#!/usr/bin/env bash

WIDTH=300
HEIGHT=300

for f in *.svg; do
    out=$(basename "$f" .svg)
    inkscape -w "$WIDTH" -h "$HEIGHT" "$f" -o "$out.png"
done
