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
cargo run -- --config ${config_file} --output ./tmp vpn --message-vpn ${rnd_vpn} --fetch
test -f ./tmp/${rnd_vpn}/vpn/${rnd_vpn}.yaml

# queue
cargo run -- --config ${config_file} queue --file examples/queue1.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue2.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue3.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue4.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue5.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} --output ./tmp queue --message-vpn ${rnd_vpn} --fetch --queue queue1
test -f ./tmp/${rnd_vpn}/queue/queue1.yaml

# queue continued
cargo run -- --config ${config_file} queue --fetch --queue queue1 --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue1.yaml --message-vpn ${rnd_vpn} --update --queue queue1
cargo run -- --config ${config_file} queue --update --shutdown --queue queue1 --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue1.yaml  --update --shutdown --queue queue1 --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --file examples/queue1.yaml  --update --no-shutdown --queue queue1 --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} queue --update --queue queue1 --message-vpn ${rnd_vpn}

# create 6-99 queues
i=6
while [ $i -lt 100 ]; do
    cargo run -- --config ${config_file} queue --file examples/queue${i}.yaml --message-vpn ${rnd_vpn} --queue queue${i}
    ((i=$i+1))
done

# acl
cargo run -- --config ${config_file} acl-profile --file examples/acl.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} acl-profile --fetch --acl-profile myacl --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} acl-profile --file examples/acl.yaml --message-vpn ${rnd_vpn} --update --acl-profile myacl
cargo run -- --config ${config_file} --output ./tmp acl-profile --message-vpn ${rnd_vpn} --fetch --acl-profile myacl
test -f ./tmp/${rnd_vpn}/acl/myacl.yaml

# client-profile
cargo run -- --config ${config_file} client-profile --file examples/client-profile.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} client-profile --fetch --client-profile myclientprofile --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} client-profile --file examples/client-profile.yaml --message-vpn ${rnd_vpn} --update --client-profile myclientprofile
cargo run -- --config ${config_file} --output ./tmp client-profile --message-vpn ${rnd_vpn} --fetch --client-profile myclientprofile
test -f ./tmp/${rnd_vpn}/client-profile/myclientprofile.yaml

# client-username
cargo run -- --config ${config_file} client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} client-username --fetch --client-username myusername --message-vpn ${rnd_vpn}
cargo run -- --config ${config_file} client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn} --update --client-username myusername
cargo run -- --config ${config_file} client-username --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown
cargo run -- --config ${config_file} client-username --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown --no-shutdown
cargo run -- --config ${config_file} client-username --message-vpn ${rnd_vpn} --update --client-username myusername --no-shutdown
cargo run -- --config ${config_file} client-username --file examples/client-username.yaml --message-vpn ${rnd_vpn} --update --client-username myusername --shutdown
cargo run -- --config ${config_file} --output ./tmp client-username --message-vpn ${rnd_vpn} --fetch --client-username myusername
test -f ./tmp/${rnd_vpn}/client-username/myusername.yaml

#exit 0

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

i=6
while [ $i -lt 100 ]; do
    cargo run -- --config ${config_file} queue --message-vpn ${rnd_vpn} --queue queue${i} --delete
    ((i=$i+1))
done

# delete vpn
cargo run -- --config ${config_file} vpn --message-vpn ${rnd_vpn} --delete

