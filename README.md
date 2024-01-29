# ocpp-binding-rs
OCPP Rust afb binding

# Debug
wireshark -i eth0 -k -S -f "host ocpp.biastaging.com and tcp port 80"


# Bug/Feature Biapower
 - do not accept connector-id=0 (should set tid=1 in config)
 - do not accept cross heartbeat (should set tic=0 in config)
 - shutdown websocket connection for any mismatch/invalid request