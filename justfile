#!/usr/bin/env just --justfile

start year day:
    cp -r .template {{year}}/day-{{day}} & \
    code {{year}}/day-{{day}}
