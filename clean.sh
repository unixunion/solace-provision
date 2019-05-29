#!/usr/bin/env bash

echo "config: $1"
echo "lang: $2"

config_file=$1
rnd_vpn=$2

if [[ "$2" == "" ]]; then
    echo "please specify config file and a vpn name to create, update and delete"
    exit 1
fi

export RUSTFLAGS=-Awarnings
export RUST_LOG=info

if [ -x "target/release/solace-provision" ]; then
    bin="target/release/solace-provision"
else
    bin="cargo run --"
fi


delete() {

    $bin --config ${config_file} auth-group --message-vpn ${rnd_vpn} --delete --auth-group myauthgroup
    # topic endpoont
    $bin --config ${config_file} topic-endpoint --message-vpn ${rnd_vpn} --update --topic-endpoint mytopic --delete
    # sequenced topic
    $bin --config ${config_file} sequenced-topic --message-vpn ${rnd_vpn} --delete --sequenced-topic "mytopic"
    # delete client-username
    $bin --config ${config_file} client-username --delete --message-vpn ${rnd_vpn} --update --client-username myusername
    # delete client-profile
    $bin --config ${config_file} client-profile --delete --message-vpn ${rnd_vpn} --update --client-profile myclientprofile
    # delete acl
    $bin --config ${config_file} acl-profile --delete --message-vpn ${rnd_vpn} --update --acl-profile myacl
    # delete queue subscription
    $bin --config ${config_file} queue-subscription --message-vpn ${rnd_vpn} --queue queue1 --delete --subscription mytopic

    # delete queue
    $bin --config ${config_file} queue --queue queue5 --message-vpn ${rnd_vpn} --delete
    $bin --config ${config_file} queue --queue queue4 --message-vpn ${rnd_vpn} --delete
    $bin --config ${config_file} queue --queue queue3 --message-vpn ${rnd_vpn} --delete
    $bin --config ${config_file} queue --queue queue2 --message-vpn ${rnd_vpn} --delete
    $bin --config ${config_file} queue --queue queue1 --message-vpn ${rnd_vpn} --delete

    i=6
    while [ $i -lt 22 ]; do
        $bin --config ${config_file} queue --message-vpn ${rnd_vpn} --queue queue${i} --delete
        ((i=$i+1))
    done

    # delete bridge
    $bin --config ${config_file} bridge --message-vpn ${rnd_vpn} --delete --bridge mybridge --virtual-router primary

    # delete vpn
    $bin --config ${config_file} vpn --message-vpn ${rnd_vpn} --delete

}

set +e
delete
set -e