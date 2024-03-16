#!/bin/bash

cd $(git rev-parse --show-toplevel)

cargo build

./target/debug/waffle_cooker

