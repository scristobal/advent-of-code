#!/usr/bin/env just --justfile

# start a new AoC day using template, eg. $ just start 2023 04
start year day:
    cp -r .template {{year}}/day-{{day}}
    touch {{year}}/day-{{day}}/input.txt
    code {{year}}/day-{{day}}

