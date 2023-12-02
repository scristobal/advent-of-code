#!/usr/bin/env just --justfile

start year day:
    cp -r .template {{year}}/day-{{day}} & \
    touch {{year}}/day-{{day}}/input.txt
    code {{year}}/day-{{day}}
