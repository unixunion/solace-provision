#!/bin/bash




echo "config: $1"
echo "lang: $2"
echo "stop_on_error: $3"

config_file=$1
rnd_vpn=$2
if [[ "$3" == "" ]]; then
    set -e
fi

if [[ "$2" == "" ]]; then
    echo "please specify config file and a vpn name to create, update and delete"
    exit 1
fi

export RUSTFLAGS=-Awarnings

# VPN commands
cargo run -- --config ${config_file} vpn --file examples/vpn.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} vpn --fetch --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} vpn --update --message-vpn ${rnd_vpn} --shutdown
cargo run -- --config ${config_file} vpn --update --message-vpn ${rnd_vpn} --shutdown --file examples/vpn.yaml
cargo run -- --config ${config_file} vpn --update --message-vpn ${rnd_vpn} --shutdown --file examples/vpn.yaml --no-shutdown

# queue
cargo run -- --config ${config_file} queue --file examples/queue1.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue2.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue3.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue4.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue5.yaml --message-vpn ${rnd_vpn}

cargo run -- --config ${config_file} queue --fetch --queue queue1 --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue1.yaml --message-vpn ${rnd_vpn} --update --queue queue1
cargo run -- --config ${config_file} queue --update --shutdown --queue queue1 --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue1.yaml  --update --shutdown --queue queue1 --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue1.yaml  --update --no-shutdown --queue queue1 --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --update --queue queue1 --message-vpn ${rnd_vpn}


# acl
cargo run -- --config ${config_file} acl-profile --file examples/acl.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} acl-profile --fetch --acl-profile myacl --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} acl-profile --file examples/acl.yaml --message-vpn ${rnd_vpn} --update --acl-profile myacl

# client-profile
cargo run -- --config ${config_file} client-profile --file examples/client-profile.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} client-profile --fetch --client-profile myclientprofile --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} client-profile --file examples/client-profile.yaml --message-vpn ${rnd_vpn} --update --client-profile myclientprofile

# client-username
cargo run -- --config ${config_file} client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} client-username --fetch --client-username myusername --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn} --update --client-username myusername
cargo run -- --config ${config_file} client-username --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown
cargo run -- --config ${config_file} client-username --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown --no-shutdown
cargo run -- --config ${config_file} client-username --message-vpn ${rnd_vpn} --update --client-username myusername --no-shutdown
cargo run -- --config ${config_file} client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown

exit 0


# delete client-username
cargo run -- --config ${config_file} client-username --delete --message-vpn ${rnd_vpn} --update --client-username myusername
# delete client-profile
cargo run -- --config ${config_file} client-profile --delete --message-vpn ${rnd_vpn} --update --client-profile myclientprofile
# delete acl
cargo run -- --config ${config_file} acl-profile --delete --message-vpn ${rnd_vpn} --update --acl-profile myacl
# delete queue
cargo run -- --config ${config_file} queue --queue queue5 --message-vpn ${rnd_vpn} --delete
cargo run -- --config ${config_file} queue --queue queue4 --message-vpn ${rnd_vpn} --delete
cargo run -- --config ${config_file} queue --queue queue3 --message-vpn ${rnd_vpn} --delete
cargo run -- --config ${config_file} queue --queue queue2 --message-vpn ${rnd_vpn} --delete
cargo run -- --config ${config_file} queue --queue queue1 --message-vpn ${rnd_vpn} --delete

# delete vpn
cargo run -- --config ${config_file} vpn --message-vpn ${rnd_vpn} --delete

