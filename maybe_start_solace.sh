#!/usr/bin/env bash

res=`docker-compose ps solace-http|grep solace_http`
if [ "$res" != "" ]; then
    echo "Solace is already running"

else
    echo "No Solace Found"
    docker-compose up -d
    docker-compose events | ./wait_for_solace_up.sh
fi
