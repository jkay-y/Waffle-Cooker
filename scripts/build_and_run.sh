#!/bin/bash

cd $(git rev-parse --show-toplevel)

if [ ! -d "build" ]; then
    mkdir build
fi

cd build

zig build-exe ../src/main.zig

cd $(git rev-parse --show-toplevel)
