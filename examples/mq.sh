#!/usr/bin/env bash
c=8
while [ $c -lt 100 ]; do
	cat queue1.yaml|sed "s/queue1/queue${c}/g" > queue${c}.yaml
    ((c=$c+1))
done
