#!/bin/bash

# use libafb development version if any
export LD_LIBRARY_PATH="/usr/local/lib64:$LD_LIBRARY_PATH"
export PATH="/usr/local/lib64:$PATH"
clear

if ! test -f $CARGO_TARGET_DIR/debug/libafb_ocpp.so; then
    echo "FATAL: missing libafb_ocpp.so use: cargo build"
    exit 1
fi

# start binder with test config
afb-binder -v \
   --config=afb-binding/etc/binder-ocpp.json \
   --config=afb-test/etc/binding-bia-power.json \
   --config=afb-test/etc/binding-test-ocpp-16.json \
   $*
