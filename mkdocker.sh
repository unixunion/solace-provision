#!/bin/bash

set -e

docker run --name rust rust:1.33 /bin/true

mkdir -p target/release/usr/lib/x86_64-linux-gnu/
mkdir -p target/release/lib64
mkdir -p target/release/lib/x86_64-linux-gnu/

docker cp rust:/usr/lib/x86_64-linux-gnu/libssl.so.1.1 target/release/usr/lib/x86_64-linux-gnu/
docker cp rust:/usr/lib/x86_64-linux-gnu/libcrypto.so.1.1 target/release/usr/lib/x86_64-linux-gnu/
docker cp rust:/lib/x86_64-linux-gnu/libdl.so.2 target/release/lib/x86_64-linux-gnu/
docker cp rust:/lib/x86_64-linux-gnu/librt.so.1 target/release/lib/x86_64-linux-gnu/
docker cp rust:/lib/x86_64-linux-gnu/libpthread.so.0 target/release/lib/x86_64-linux-gnu/
docker cp rust:/lib/x86_64-linux-gnu/libgcc_s.so.1 target/release/lib/x86_64-linux-gnu/
docker cp rust:/lib64/ld-linux-x86-64.so.2 target/release/lib64
docker cp rust:/lib/x86_64-linux-gnu/libm.so.6 target/release/lib/x86_64-linux-gnu/

docker stop rust
docker rm rust

echo "done"
