# OCPP binding
===============

## Provides

* Rust implementation of OCPP charger stack
* afb-binding micro service architecture and security model [api-v4](https://github.com/redpesk-common/afb-librust/blob/master/docs/1-architecture_presentation.md)-

## References

Specification from OpenCharge Alliance https://www.openchargealliance.org/downloads

* OCPP-1.6 ocpp-j-1.6-specification.pdf
* OCPP-2.0.1 https://www.oasis-open.org/committees/download.php/70384/OCPP-2.0.1_part4_ocpp-j-specification.pdf

This code is freely inspired from differences open-sources references:

* NodeJs https://github.com/mikuso/ocpp-rpc
* Python https://github.com/mobilityhouse/ocpp


## Sample connecting Flow

* Connect WS http/upgrade ocpp16|occp201
   * provide adequate authentication (basic-auth, tls, ...)

* Boot notification (frontend -> backend)
   * {"BootNotification",{"chargePointVendor":"Tux-EvSe","chargePointModel":"Dummy testing client"}}
   * {"currentTime":"2023-09-29T11:42:31Z","interval":300,"status":"Accepted"}

* Status notification (frontend -> backend)
  * {"StatusNotification",{"connectorId":1,"errorCode":"NoError","status":"Available"}}
  * {}

* Data transfert (frontend -> backend)
  * {"DataTransfert", {"vendorId":"Bia","data":"Bia Power Grid SL. This is an emulated frontend"}}
  * {"status":"Accepted","data":"Bia Power Grid SL. This is an emulated frontend"}


## testing server connectivity

### AFB-client

```
# direct access to ocpp-backend
afb-client -d unix:@OCPP-SND
 Heartbeat {}
 Heartbeat-xxx {}

# access through afb-ocpp-binding
afb-client localhost:1234/api
 Heartbeat {}
```


### Debug
Warning: user should be in wireshark group
```
wireshark -i eth0 -k -S -f "host ocpp.biastaging.com and tcp port 80"&
./afb-test/etc/binding-test-ocpp-16.sh
```


### Bug/Feature Biapower
 - do not accept connector-id=0 (should set tid=1 in config)
 - do not accept cross heartbeat (should set tic=0 in config)
 - shutdown websocket connection for any mismatch/invalid request
 - keep sending charging profile even when ```json {"response":{"status":"accepted"}}```
 - dashboard web-ui is very slow to respond (require extra delay between test)

