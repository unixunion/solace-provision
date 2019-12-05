#!/bin/bash

export RUST_BACKTRACE=1

# vpn
cargo run -- --config examples/config_solace1.yaml vpn --message-vpn myvpn --delete
cargo run -- --config examples/config_solace1.yaml vpn --file examples/vpn.yaml
cargo run -- --config examples/config_solace1.yaml vpn --file examples/vpn.yaml --update
cargo run -- --config examples/config_solace1.yaml vpn --fetch --message-vpn "*"
cargo run -- --config examples/config_solace1.yaml vpn --shutdown --message-vpn myvpn
cargo run -- --config examples/config_solace1.yaml vpn --no-shutdown --message-vpn myvpn
cargo run -- --config examples/config_solace1.yaml vpn --file examples/vpn.yaml --update --shutdown --message-vpn myvpn --no-shutdown

# acl
cargo run -- --config examples/config_solace1.yaml acl-profile --file examples/acl.yaml
cargo run -- --config examples/config_solace1.yaml acl-profile --file examples/acl.yaml --update
cargo run -- --config examples/config_solace1.yaml acl-profile  --fetch --message-vpn myvpn --acl-profile "*"
cargo run -- --config examples/config_solace1.yaml acl-profile  --message-vpn myvpn --acl-profile "myacl" --delete

# bridge
cargo run -- --config examples/config_solace1.yaml bridge --file examples/bridge-primary.yaml
cargo run -- --config examples/config_solace1.yaml bridge --file examples/bridge-primary.yaml --update
cargo run -- --config examples/config_solace1.yaml bridge --fetch --message-vpn myvpn --bridge mybridge
cargo run -- --config examples/config_solace1.yaml bridge --shutdown --message-vpn myvpn --bridge mybridge
cargo run -- --config examples/config_solace1.yaml bridge --no-shutdown --message-vpn myvpn --bridge mybridge
cargo run -- --config examples/config_solace1.yaml bridge --message-vpn myvpn --update --file examples/bridge-primary.yaml --bridge mybridge --shutdown --no-shutdown

# client profile
cargo run -- --config examples/config_solace1.yaml client-profile --file examples/client-profile.yaml --message-vpn myvpn

# remote-brige
cargo run -- --config examples/config_solace1.yaml remote-bridge --file examples/bridge-remote-primary.yaml
cargo run -- --config examples/config_solace1.yaml remote-bridge --file examples/bridge-remote-primary.yaml --update
cargo run -- --config examples/config_solace1.yaml remote-bridge --fetch --message-vpn myvpn --bridge mybridge --virtual-router primary
cargo run -- --config examples/config_solace1.yaml remote-bridge --shutdown --message-vpn myvpn --bridge mybridge --virtual-router primary
cargo run -- --config examples/config_solace1.yaml remote-bridge --no-shutdown --message-vpn myvpn --bridge mybridge --virtual-router primary
cargo run -- --config examples/config_solace1.yaml remote-bridge --file examples/bridge-remote-primary.yaml --update --shutdown --no-shutdown --message-vpn myvpn --bridge mybridge --virtual-router primary

# delete remote-brige
cargo run -- --config examples/config_solace1.yaml remote-bridge --bridge mybridge --delete --message-vpn myvpn --virtual-router primary

# replay-log
cargo run -- --config examples/config_solace1.yaml replay-log --file examples/replay.yaml
cargo run -- --config examples/config_solace1.yaml replay-log --file examples/replay.yaml --update
cargo run -- --config examples/config_solace1.yaml replay-log --fetch --message-vpn myvpn  --replay-log myreplaylog
cargo run -- --config examples/config_solace1.yaml replay-log --message-vpn myvpn --replay-log myreplaylog --shutdown
cargo run -- --config examples/config_solace1.yaml replay-log --message-vpn myvpn --replay-log myreplaylog --no-shutdown-egress
cargo run -- --config examples/config_solace1.yaml replay-log --message-vpn myvpn --replay-log myreplaylog --no-shutdown-ingress
cargo run -- --config examples/config_solace1.yaml replay-log --message-vpn myvpn --replay-log myreplaylog --shutdown-egresss
cargo run -- --config examples/config_solace1.yaml replay-log --message-vpn myvpn --replay-log myreplaylog --shutdown-ingress
cargo run -- --config examples/config_solace1.yaml replay-log --message-vpn myvpn --replay-log myreplaylog --no-shutdown
cargo run -- --config examples/config_solace1.yaml replay-log --message-vpn myvpn --replay-log myreplaylog --delete

# dmr-bridge
cargo run -- --config examples/config_solace1.yaml dmr-bridge --file examples/dmr-bridge.yaml
cargo run -- --config examples/config_solace1.yaml dmr-bridge --fetch --message-vpn default --remote-message-vpn "*"
cargo run -- --config examples/config_solace1.yaml dmr-bridge --delete --message-vpn default --remote-node-name solace1

# dmr-cluster
cargo run -- --config examples/config_solace1.yaml dmr-cluster --file examples/dmr-cluster.yaml
cargo run -- --config examples/config_solace1.yaml dmr-cluster --fetch --cluster-name "*"
cargo run -- --config examples/config_solace1.yaml dmr-cluster --update --file examples/dmr-cluster.yaml
cargo run -- --config examples/config_solace1.yaml dmr-cluster --delete --cluster-name mydmr

# cleanups
cargo run -- --config examples/config_solace1.yaml client-profile --message-vpn myvpn --client-profile myclientprofile --delete
cargo run -- --config examples/config_solace1.yaml bridge --message-vpn myvpn --bridge mybridge --delete --virtual-router primary
cargo run -- --config examples/config_solace1.yaml vpn --message-vpn myvpn --delete

