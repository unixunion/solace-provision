#!/usr/bin/env bash

# build native
if [ -d "/src" ]; then
    cd /src
fi

cargo build --release
cp /lib/x86_64-linux-gnu/libgcc_s.so.1 target/release
cp /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 target/release
