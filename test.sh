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

# queue
cargo run -- --config examples/config.yaml queue --file examples/queue.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml queue --fetch --queue myqueue --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml queue --file examples/queue.yaml --message-vpn ${rnd_vpn} --update --queue myqueue
cargo run -- --config examples/config.yaml queue --update --shutdown --queue myqueue --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml queue --file examples/queue.yaml  --update --shutdown --queue myqueue --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml queue --file examples/queue.yaml  --update --no-shutdown --queue myqueue --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml queue --update --queue myqueue --message-vpn ${rnd_vpn}


# acl
cargo run -- --config examples/config.yaml acl-profile --file examples/acl.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml acl-profile --fetch --acl-profile myacl --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml acl-profile --file examples/acl.yaml --message-vpn ${rnd_vpn} --update --acl-profile myacl

# client-profile
cargo run -- --config examples/config.yaml client-profile --file examples/client-profile.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml client-profile --fetch --client-profile myclientprofile --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml client-profile --file examples/client-profile.yaml --message-vpn ${rnd_vpn} --update --client-profile myclientprofile


# client-username
cargo run -- --config examples/config.yaml client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml client-username --fetch --client-username myusername --message-vpn ${rnd_vpn}
cargo run -- --config examples/config.yaml client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn} --update --client-username myusername
cargo run -- --config examples/config.yaml client-username --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown
cargo run -- --config examples/config.yaml client-username --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown --no-shutdown
cargo run -- --config examples/config.yaml client-username --message-vpn ${rnd_vpn} --update --client-username myusername --no-shutdown
cargo run -- --config examples/config.yaml client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown




# delete client-username
cargo run -- --config examples/config.yaml client-username --delete --message-vpn ${rnd_vpn} --update --client-username myusername
# delete client-profile
cargo run -- --config examples/config.yaml client-profile --delete --message-vpn ${rnd_vpn} --update --client-profile myclientprofile
# delete acl
cargo run -- --config examples/config.yaml acl-profile --delete --message-vpn ${rnd_vpn} --update --acl-profile myacl
# delete queue
cargo run -- --config examples/config.yaml queue --queue myqueue --message-vpn ${rnd_vpn} --delete
# delete vpn
cargo run -- --config examples/config.yaml vpn --message-vpn ${rnd_vpn} --delete

