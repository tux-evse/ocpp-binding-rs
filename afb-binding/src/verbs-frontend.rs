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
use typesv4::prelude::*;

// init ocpp backend at API initialization time
pub fn ocpp_bootstrap(api: &AfbApi, station: &str, tic: u32) -> Result<(), AfbError> {
    AfbSubCall::call_sync(
        api,
        "OCPP-C",
        "BootNotification",
        v106::BootNotification::Request(v106::BootNotificationRequest {
            charge_point_vendor: station.to_string(),
            charge_point_model: "Tux-Evse OCPP-1.6".to_string(),
            firmware_version: Some("v1234".to_string()),
            charge_box_serial_number: None,
            charge_point_serial_number: None,
            iccid: None,
            imsi: None,
            meter_serial_number: None,
            meter_type: None,
        }),
    )?;

    AfbSubCall::call_sync(
        api,
        "OCPP-C",
        "StatusNotification",
        v106::StatusNotification::Request(v106::StatusNotificationRequest {
            connector_id: 1,
            status: v106::ChargePointStatus::Preparing,
            error_code: v106::ChargePointErrorCode::NoError,
            info: Some("Tux-Evse booting report".to_string()),
            timestamp: Some(get_utc()),
            vendor_id: None,
            vendor_error_code: None,
        }),
    )?;

    AfbSubCall::call_sync(
        api,
        "OCPP-C",
        "Heartbeat",
        v106::Heartbeat::Request(v106::HeartbeatRequest {}),
    )?;

    if tic > 0 {
        AfbTimer::new("tic-timer")
            .set_period(tic)
            .set_decount(0)
            .set_callback(Box::new(TimerCtx {
                apiv4: api.get_apiv4(),
            }))
            .start()?;
    }
    Ok(())
}

struct TimerCtx {
    apiv4: AfbApiV4,
}
// ping server every tic-ms to keep ocpp connection live (Warning: not supported by biapower backend)
AfbTimerRegister!(TimerCtrl, timer_cb, TimerCtx);
fn timer_cb(_timer: &AfbTimer, _decount: u32, ctx: &mut TimerCtx) -> Result<(), AfbError> {
    AfbSubCall::call_sync(
        ctx.apiv4,
        "OCPP-C",
        "Heartbeat",
        v106::Heartbeat::Request(v106::HeartbeatRequest {}),
    )?;
    Ok(())
}

AfbCallRegister!(MeterValuesRsp, meter_values_response);
fn meter_values_response(_api: &AfbApi, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::MeterValues>(0)?;
    match data {
        v106::MeterValues::Response(_response) => {}
        _ => return afb_error!("ocpp-metervalue-rsp", "invalid response type"),
    };
    Ok(())
}

// ref: https://www.ampcontrol.io/ocpp-guide/how-to-send-ocpp-meter-values-with-metervalues-req
fn engy_event_action(state: &EnergyState, mgr: &'static ManagerHandle ) -> Result<v106::MeterValuesRequest , AfbError> {

    let tid = mgr.get_tid()?;
    if tid == 0 {
        return afb_error!("ocpp-energy-state", "not active transaction running");
    }

    let tension_value = v106::SampledValue {
        value: (state.tension / 100).to_string(),
        location: None, // string
        context: None,  // string
        phase: None,    // default L1
        format: None,   // default raw
        unit: Some(v106::UnitOfMeasure::V),
        measurand: Some(v106::Measurand::Voltage),
    };

    let current_value = v106::SampledValue {
        value: (state.current / 100).to_string(),
        location: None, // string
        context: None,  // string
        phase: None,    // default L1
        format: None,   // default Raw
        unit: Some(v106::UnitOfMeasure::A),
        measurand: Some(v106::Measurand::CurrentImport),
    };

    let power_value = v106::SampledValue {
        value: (state.power / 100).to_string(),
        location: None, // string
        context: None,  // string
        phase: None,    // default L1
        format: None,   // default Raw
        unit: Some(v106::UnitOfMeasure::Kw),
        measurand: Some(v106::Measurand::PowerActiveImport),
    };

    let session_value = v106::SampledValue {
        value: (state.session / 100).to_string(),
        location: None, // string
        context: None,  // string
        phase: None,    // default L1
        format: None,   // default Raw
        unit: Some(v106::UnitOfMeasure::KWh),
        measurand: Some(v106::Measurand::EnergyActiveImportRegister),
    };

    let query = v106::MeterValuesRequest {
        connector_id: mgr.get_cid(),
        transaction_id: Some(tid),
        meter_value: vec![v106::MeterValue {
            timestamp: get_utc(),
            sampled_value: vec![tension_value, power_value, current_value, session_value],
        }],
    };


    Ok(query)
}

struct EngyEvtCtx {
    mgr: &'static ManagerHandle,
}
// report value meter to ocpp backend
AfbEventRegister!(EngyEvtCtrl, engy_event_cb, EngyEvtCtx);
fn engy_event_cb(evt: &AfbEventMsg, args: &AfbData, ctx: &mut EngyEvtCtx) -> Result<(), AfbError> {

    let state = args.get::<&EnergyState>(0)?;
    afb_log_msg!(Debug, evt, "energy:{:?}", state.clone());
    let query= engy_event_action(state, ctx.mgr) ?;

    AfbSubCall::call_async(
        evt.get_apiv4(),
        "OCPP-C",
        "MeterValues",
        v106::MeterValues::Request(query),
        Box::new(MeterValuesRsp {}),
    )?;
    Ok(())
}

struct EngyMockRqtCtx {
    mgr: &'static ManagerHandle,
}
// this verb is only for testing purpose real measure should be send from engy event
AfbVerbRegister!(EngyMockRqt, engy_state_request, EngyMockRqtCtx);
fn engy_state_request(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut EngyMockRqtCtx,
) -> Result<(), AfbError> {
    let state = args.get::<&EnergyState>(0)?;

    afb_log_msg!(Debug, rqt, "EngyStateVerb request");
    let query= engy_event_action( state, ctx.mgr) ?;

    AfbSubCall::call_sync(
        rqt.get_api().get_apiv4(),
        "OCPP-C",
        "MeterValues",
        v106::MeterValues::Request(query),
    )?;
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct HeartbeatRspCtx {
    nonce: u32,
}
AfbVerbRegister!(HeartbeatRsp, heartbeat_response, HeartbeatRspCtx);
fn heartbeat_response(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut HeartbeatRspCtx,
) -> Result<(), AfbError> {
    let data = args.get::<&v106::Heartbeat>(0)?;
    let response = match data {
        v106::Heartbeat::Response(response) => response,
        _ => return afb_error!("ocpp-heartbeat-response", "invalid response type"),
    };

    afb_log_msg!(
        Debug,
        rqt,
        "Heartbeat request nonce:{} time={}",
        ctx.nonce,
        response.current_time
    );
    rqt.reply(ctx.nonce, 0);
    Ok(())
}

// Authentication check id_tag on backend
AfbVerbRegister!(HeartbeatRqt, heartbeat_request);
fn heartbeat_request(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let nonce = args.get::<u32>(0)?;
    afb_log_msg!(Debug, rqt, "Heartbeat request nonce:{}", nonce);

    let query = v106::HeartbeatRequest {};
    AfbSubCall::call_async(
        rqt,
        "OCPP-C",
        "Heartbeat",
        v106::Heartbeat::Request(query),
        Box::new(HeartbeatRspCtx { nonce }),
    )?;
    Ok(())
}

// Authorize async start response callback
struct AuthorizeRspCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(AuthorizeRsp, authorize_response, AuthorizeRspCtx);
fn authorize_response(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut AuthorizeRspCtx,
) -> Result<(), AfbError> {
    let data = args.get::<&v106::Authorize>(0)?;
    let response = match data {
        v106::Authorize::Response(response) => response,
        _ => return afb_error!("ocpp-authorize-start", "invalid response type"),
    };

    match response.id_tag_info.status {
        v106::AuthorizationStatus::Accepted => {
            afb_log_msg!(Debug, rqt, "ocpp-authorize-done");
            ctx.mgr.authorized(true)?;
        }
        _ => {
            return afb_error!(
                "ocpp-authorize-start",
                "fail auth:{:?}",
                response.id_tag_info.status
            )
        }
    };
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

// Authentication check id_tag on backend
struct AuthorizeRqtCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(AuthorizeRqt, authorize_request, AuthorizeRqtCtx);
fn authorize_request(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut AuthorizeRqtCtx,
) -> Result<(), AfbError> {
    let id_tag = args.get::<String>(0)?;

    afb_log_msg!(Debug, rqt, "Authorize request tag:{}", id_tag);
    let query = v106::AuthorizeRequest { id_tag };

    AfbSubCall::call_async(
        rqt,
        "OCPP-C",
        "Authorize",
        v106::Authorize::Request(query),
        Box::new(AuthorizeRspCtx { mgr: ctx.mgr }),
    )?;
    Ok(())
}

// Transaction async start response callback
struct TransacStartRspCtx {
    mgr: &'static ManagerHandle,
}
// reference: https://www.ampcontrol.io/ocpp-guide/how-to-start-an-ocpp-charging-session-with-starttransaction
AfbVerbRegister!(TransacStartRsp, transac_start_rsp, TransacStartRspCtx);
fn transac_start_rsp(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut TransacStartRspCtx,
) -> Result<(), AfbError> {
    let data = args.get::<&v106::StartTransaction>(0)?;
    let response = match data {
        v106::StartTransaction::Response(response) => response,
        _ => return afb_error!("ocpp-transaction-start", "invalid response"),
    };

    let tid = match response.id_tag_info.status {
        v106::AuthorizationStatus::Accepted => response.transaction_id,
        _ => {
            return afb_error!(
                "ocpp-transaction-start",
                "fail auth:{:?}",
                response.id_tag_info.status
            )
        }
    };
    ctx.mgr.login(tid)?;
    rqt.reply(tid, 0);
    Ok(())
}

// Transaction stop async response callback
struct TransacStopRspCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(TransacStopRsp, transac_stop_rsp, TransacStopRspCtx);
fn transac_stop_rsp(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut TransacStopRspCtx,
) -> Result<(), AfbError> {
    let data = args.get::<&v106::StopTransaction>(0)?;
    match data {
        v106::StopTransaction::Response(response) => response,
        _ => return afb_error!("ocpp-transaction-stop", "invalid response type"),
    };

    ctx.mgr.logout()?;
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

// Transaction start/stop request
struct TransacRqtCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(TransacRequest, transaction_request, TransacRqtCtx);
fn transaction_request(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut TransacRqtCtx,
) -> Result<(), AfbError> {
    let data = args.get::<&OcppTransaction>(0)?;
    match data {
        OcppTransaction::Start(tag) => {
            afb_log_msg!(Debug, rqt, "Start Transaction request");
            ctx.mgr.check_active_session(false)?;
            let query = v106::StartTransactionRequest {
                connector_id: ctx.mgr.get_cid(),
                id_tag: tag.clone(),
                meter_start: 0,
                reservation_id: None,
                timestamp: get_utc(),
            };

            AfbSubCall::call_async(
                rqt,
                "OCPP-C",
                "StartTransaction",
                v106::StartTransaction::Request(query),
                Box::new(TransacStartRspCtx { mgr: ctx.mgr }),
            )?;
        }
        OcppTransaction::Stop(meter) => {
            afb_log_msg!(Debug, rqt, "Stop Transaction request");
            ctx.mgr.check_active_session(true)?;
            let query = v106::StopTransactionRequest {
                id_tag: None,
                meter_stop: *meter,
                timestamp: get_utc(),
                reason: None,
                transaction_data: None,
                transaction_id: ctx.mgr.get_tid()?,
            };

            AfbSubCall::call_async(
                rqt,
                "OCPP-C",
                "StopTransaction",
                v106::StopTransaction::Request(query),
                Box::new(TransacStopRspCtx { mgr: ctx.mgr }),
            )?;
        }
    }
    Ok(())
}

pub(crate) fn register_frontend(api: &mut AfbApi, config: &BindingConfig) -> Result<(), AfbError> {
    let engy_handler = AfbEvtHandler::new("energy-evt")
        .set_pattern(to_static_str(format!("{}/*", config.engy_api)))
        .set_callback(Box::new(EngyEvtCtx { mgr: config.mgr }))
        .finalize()?;

    let heartbeat_verb = AfbVerb::new("heartbeat")
        .set_callback(Box::new(HeartbeatRqt {}))
        .set_info("Request tagid authorization from backend")
        .finalize()?;

    let authorize_verb = AfbVerb::new("authorize")
        .set_callback(Box::new(AuthorizeRqtCtx { mgr: config.mgr }))
        .set_info("Request tagid authorization from backend")
        .set_sample("'tux-evse-1'")?
        .set_sample("'tux-evse-2'")?
        .set_usage("idTag")
        .finalize()?;

    let transaction_verb = AfbVerb::new("transaction")
        .set_callback(Box::new(TransacRqtCtx { mgr: config.mgr }))
        .set_info("send start/stop transaction to backend")
        .set_usage("'idTag'")
        .finalize()?;

    let engy_state_verb = AfbVerb::new("engy-state")
        .set_callback(Box::new(EngyMockRqtCtx { mgr: config.mgr }))
        .set_info("testing vern to mock engy state event")
        .finalize()?;


    // register veb within API
    api.add_verb(authorize_verb);
    api.add_verb(transaction_verb);
    api.add_verb(engy_state_verb);
    api.add_verb(heartbeat_verb);
    api.add_evt_handler(engy_handler);

    Ok(())
}
