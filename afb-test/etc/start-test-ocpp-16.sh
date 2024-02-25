#!/bin/bash

export LD_LIBRARY_PATH=/usr/local/lib64
pkill afb-ocpp
cynagora-admin set '' 'HELLO' '' '*' yes
clear

# build test config dirname
DIRNAME=`dirname $0`
cd $DIRNAME/..
ROOTDIR=`pwd`
CONFDIR=`pwd`/etc
mkdir -p /tmp/api

DEVTOOL_PORT=1235
echo Ocpp debug mode config=$CONFDIR/*.json port=$DEVTOOL_PORT
echo "clear && cargo build --example ocpp_test"

afb-binder --name=afb-ocpp --port=$DEVTOOL_PORT -v \
  --config=$ROOTDIR/../afb-binding/etc/binder-ocpp.json \
  --config=$CONFDIR/binding-bia-power.json \
  --config=$CONFDIR/binding-test-ocpp-16.json \
  $*