#!/bin/bash

cd $(git rev-parse --show-toplevel)

if [ ! -d "build" ]; then
    mkdir build
fi

cd build

cmake ..
make

cd $(git rev-parse --show-toplevel)

./build/WaffleCooker
