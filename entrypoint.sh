#!/usr/bin/env bash
#echo "updating CA certs from path: /usr/local/share/ca-certificates/, if you want to add certs, mount the files there."
#update-ca-certificates
#echo "using cacerts in /system/etc/security/cacerts, place your CA crt files there"
echo "running solace-provision $@"
exec /bin/solace-provision "$@"