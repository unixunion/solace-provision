#!/bin/bash

set -e

echo "arg: $1"
rnd_vpn=$1

if [ "$1" == "" ]; then
    echo "please specify a vpn name"
    exit 1
fi


export RUSTFLAGS=-Awarnings

# VPN commands
cargo run -- --config examples/config.yaml vpn --file examples/vpn.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml vpn --fetch --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml vpn --update --message-vpn ${rnd_vpn} --shutdown
cargo run -- --config examples/config.yaml vpn --update --message-vpn ${rnd_vpn} --shutdown --file examples/vpn.yaml
cargo run -- --config examples/config.yaml vpn --update --message-vpn ${rnd_vpn} --shutdown --file examples/vpn.yaml --no-shutdown
cargo run -- --config examples/config.yaml vpn --message-vpn ${rnd_vpn} --delete

