#!/bin/bash

set -e

rnd_vpn=`date +%s`

cargo run -- --config examples/config.yaml --vpn examples/vpn.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml --fetch-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml --queue examples/queue.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml --acl examples/acl.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml --client-profile examples/client-profile.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml --client-username examples/client-username.yaml --message-vpn ${rnd_vpn}

