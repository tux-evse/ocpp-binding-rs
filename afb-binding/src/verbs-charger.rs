/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 */

use crate::prelude::*;
use afbv4::prelude::*;
use ocpp::prelude::*;

struct HeartbeatCtxData {
    count: u32,
}
AfbVerbRegister!(HeartbeatVerb, heartbeat_cb, HeartbeatCtxData);
fn heartbeat_cb(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut HeartbeatCtxData,
) -> Result<(), AfbError> {
    let data = args.get::<&v106::Heartbeat>(0)?;
    match data {
        v106::Heartbeat::Response(response) => {
            afb_log_msg!(
                Debug,
                rqt,
                "OCCP-16 Heartbeat count={} {:?}",
                ctx.count,
                &response
            );
            rqt.reply(data.clone(), 0);
        }
        v106::Heartbeat::Request(_request) => {
            ctx.count = ctx.count + 1;
            afb_log_msg!(Debug, rqt, "OCCP-16 Heartbeat count={} Request", ctx.count);
            match AfbSubCall::call_async(
                rqt,
                "OCPP-C",
                "Heartbeat-xxx",
                data.clone(),
                Box::new(HeartbeatCtxData { count: ctx.count }),
            ) {
                Err(error) => {
                    afb_log_msg!(Error, rqt, &error);
                }
                Ok(()) => {}
            };
        }
    }
    Ok(())
}

AfbVerbRegister!(BootNotificationVerb, boot_notification_cb);
fn boot_notification_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::BootNotification>(0)?;
    match data {
        v106::BootNotification::Request(request) => {
            afb_log_msg!(Debug, rqt, "Got a boot_notification request {:?}", request);
            AfbSubCall::call_async(
                rqt,
                "OCPP-C",
                "BootNotification",
                data.clone(),
                Box::new(BootNotificationVerb {}),
            )?;
        }
        v106::BootNotification::Response(response) => {
            afb_log_msg!(
                Debug,
                rqt,
                "Got a boot_notification response {:?}",
                response
            );
            rqt.reply(data.clone(), 0);
        }
    }
    Ok(())
}

AfbVerbRegister!(AuthorizeVerb, authorize_cb);
fn authorize_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::Authorize>(0)?;
    match data {
        v106::Authorize::Request(request) => {
            afb_log_msg!(Debug, rqt, "Got a authorize request {:?}", request);
            AfbSubCall::call_async(
                rqt,
                "OCPP-C",
                "Authorize",
                data.clone(),
                Box::new(AuthorizeVerb {}),
            )?;
        }
        v106::Authorize::Response(response) => {
            afb_log_msg!(Debug, rqt, "Got a authorize response {:?}", response);
            rqt.reply(data.clone(), 0);
        }
    }
    Ok(())
}

AfbVerbRegister!(StatusNotificationVerb, status_notification);
fn status_notification(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::StatusNotification>(0)?;
    match data {
        v106::StatusNotification::Request(request) => {
            afb_log_msg!(Debug, rqt, "status_notification request {:?}", request);

            // send rqt to OCPP server and use same callback for data
            AfbSubCall::call_async(
                rqt,
                "OCPP_C",
                "StatusNotification",
                data.clone(),
                Box::new(BootNotificationVerb {}),
            )?;

            rqt.reply(AFB_NO_DATA, 0);
        }
        v106::StatusNotification::Response(response) => {
            afb_log_msg!(
                Warning,
                rqt,
                "status_notification response response {:?}",
                response
            );
            rqt.reply(AFB_NO_DATA, 0);
        }
    }
    Ok(())
}

AfbVerbRegister!(StartTransactionVerb, start_transaction_cb);
fn start_transaction_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::RemoteStartTransaction>(0)?;
    match data {
        v106::RemoteStartTransaction::Request(request) => {
            afb_log_msg!(Debug, rqt, "Got StartTransaction request {:?}", request);
            AfbSubCall::call_async(
                rqt,
                "OCPP-C",
                "StartTransaction",
                data.clone(),
                Box::new(StartTransactionVerb {}),
            )?;
        }
        v106::RemoteStartTransaction::Response(_response) => {
            rqt.reply(data.clone(), 0);
        }
    }
    Ok(())
}

AfbVerbRegister!(StopTransactionVerb, stop_transaction_cb);
fn stop_transaction_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::RemoteStopTransaction>(0)?;
    match data {
        v106::RemoteStopTransaction::Request(request) => {
            afb_log_msg!(Debug, rqt, "Got StartTransaction request {:?}", request);
            AfbSubCall::call_async(
                rqt,
                "OCPP-C",
                "StartTransaction",
                data.clone(),
                Box::new(StopTransactionVerb {}),
            )?;
        }
        v106::RemoteStopTransaction::Response(_response) => {
            rqt.reply(data.clone(), 0);
        }
    }
    Ok(())
}

// Fulup Verb TBD
// 6.16. DataTransfer.conf
// 6.17. DiagnosticsStatusNotification.req
// 6.19. FirmwareStatusNotification.req
// 6.31. MeterValues.req
// 6.45. StartTransaction.req
// 6.49. StopTransaction.req

AfbEventRegister!(EventGetCtrl, event_get_callback);
fn event_get_callback(event: &AfbEventMsg, args: &AfbData) -> Result<(), AfbError> {
    // check request introspection
    let evt_uid = event.get_uid();
    let evt_name = event.get_name();
    let api_uid = event.get_api().get_uid();

    afb_log_msg!(
        Notice,
        event,
        "--Got socket event evt={} name={} api={}",
        evt_uid,
        evt_name,
        api_uid
    );

    let argument = args.get::<JsoncObj>(0)?;
    afb_log_msg!(Info, event, "Got valid jsonc object argument={}", argument);

    Ok(())
}

pub(crate) fn register_charger(api: &mut AfbApi, _config: &BindingConfig) -> Result<(), AfbError> {
    let authorize = AfbVerb::new("Authorize")
        .set_callback(Box::new(AuthorizeVerb {}))
        .set_info("Request authorize config from backend")
        .set_sample("{'idTag':'tux-evse-1'}")?
        .set_sample("{'idTag':'tux-evse-2'}")?
        .set_usage("{'idTag':'xxx'}")
        .finalize()?;

    let boot_notification = AfbVerb::new("BootNotification")
        .set_callback(Box::new(BootNotificationVerb {}))
        .set_info("send boot notification to backend")
        .set_sample("{'chargePointVendor':'tux-evse', 'chargePointModel':'test1'}")?
        .set_sample("{'chargePointVendor':'tux-evse', 'chargePointModel':'test2', 'chargeBoxSerialNumber': 'XX-1234', 'firmwareVersion': 'XY-4321'}")?
        .set_usage("{'chargePointVendor':'string', 'chargePointModel':'string'}")
        .finalize()?;

    let status_notification = AfbVerb::new("StatusNotification")
        .set_callback(Box::new(StatusNotificationVerb {}))
        .set_info("send status notification to backend")
        .set_sample("{'connector_id':1, 'status':'Available', 'error_code':'NoError'}")?
        .set_sample("{'connector_id':1, 'status':'Charging', 'error_code':'NoError'}")?
        .set_sample("{'connector_id':0, 'status':'Faulted', 'error_code':'GroundFailure'}")?
        .set_sample(
            "{'connector_id':1, 'error_code':'v106::ChargePointErrorCode::PowerSwitchFailure'}",
        )?
        .set_usage("{'connector_id': int, 'error_code':'ErrorCode(enum)'}")
        .finalize()?;

    let heartbeat = AfbVerb::new("Heartbeat")
        .set_callback(Box::new(HeartbeatVerb { count: 0 }))
        .set_info("send heartbeat to backend")
        .set_sample("{}")?
        .set_usage("{/* empty object */}")
        .finalize()?;

    let start_transaction = AfbVerb::new("StartTransaction")
        .set_callback(Box::new(StartTransactionVerb {}))
        .set_info("send start transaction to backend")
        .set_sample("{'id_tag': 'Tux-EvSe-Tag', 'connector_id':1, 'charging_profile': 'None'}")?
        .set_usage("{'connector_id': 'string', 'connector_id':if(optional), 'charging_profile', ChargingProfile(optional)}")
        .finalize()?;

    let stop_transaction = AfbVerb::new("StopTransaction")
        .set_callback(Box::new(StartTransactionVerb {}))
        .set_info("send stop transaction to backend")
        .set_sample("{'id_tag': 'Tux-EvSe-Tag', 'connector_id':1, 'charging_profile': 'None'}")?
        .set_usage("{'connector_id': 'string', 'connector_id':if(optional), 'charging_profile', ChargingProfile(optional)}")
        .finalize()?;

    let websock_handler = AfbEvtHandler::new("handler-1")
        .set_info("Monitoring event handler")
        .set_pattern("*")
        .set_callback(Box::new(EventGetCtrl {}))
        .finalize()?;

    // register veb within API
    api.add_verb(authorize);
    api.add_verb(boot_notification);
    api.add_verb(status_notification);
    api.add_verb(start_transaction);
    api.add_evt_handler(websock_handler);
    api.add_verb(stop_transaction);
    api.add_verb(heartbeat);

    Ok(())
}
