#!/usr/bin/env bash

# build native
if [ -d "/src" ]; then
    cd /src
fi

cargo build --release
