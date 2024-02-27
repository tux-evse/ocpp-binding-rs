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

AfbCallRegister!(IgnoreResponseCtrl, ignore_timer_rsp);
fn ignore_timer_rsp(_api: &AfbApi, _args: &AfbData) -> Result<(), AfbError> {
    Ok(())
}

struct MonitorEvtCtx{
    mgr: &'static ManagerHandle,
}
AfbEventRegister!(MonitorEvtCtrl, monitor_event_cb,MonitorEvtCtx );
fn monitor_event_cb(evt: &AfbEventMsg, args: &AfbData, ctx: &mut MonitorEvtCtx) -> Result<(), AfbError> {
    let status = ctx.mgr.get_status()?;
    let msg = args.get::<String>(0)?;
    afb_log_msg!(
        Warning,
        evt,
        "monitor_evt ocpp server websocket reset evt:{:?} status:{:?}",
        msg, &status
    );

    let query = update_charger_status(ctx.mgr, &status)?;
    AfbSubCall::call_async(
        evt.get_apiv4(),
        "OCPP-SND",
        "StatusNotification",
        v106::StatusNotification::Request(query),
        Box::new(IgnoreResponseCtrl {}),
    )?;

    Ok(())
}

struct TimerCtx {
    apiv4: AfbApiV4,
    mgr: &'static ManagerHandle,
}
// ping server every tic-ms to keep ocpp connection live (Warning: not supported by biapower backend)
AfbTimerRegister!(TimerCtrl, timer_cb, TimerCtx);
fn timer_cb(_timer: &AfbTimer, _decount: u32, ctx: &mut TimerCtx) -> Result<(), AfbError> {

    // keep updating 'available' charger status for OCPP not to forget about us
    let status = ctx.mgr.get_status()?;
    match status {
        OcppChargerStatus::Available => {}
        _ => return Ok(()),
    }

    let query = update_charger_status(ctx.mgr, &status)?;
    AfbSubCall::call_async(
        ctx.apiv4,
        "OCPP-SND",
        "StatusNotification",
        v106::StatusNotification::Request(query),
        Box::new(IgnoreResponseCtrl {}),
    )?;
    Ok(())
}

// init ocpp backend at API initialization time
pub fn ocpp_bootstrap(
    api: &AfbApi,
    mgr: &'static ManagerHandle,
    station: &str,
    tic: u32,
) -> Result<(), AfbError> {
    AfbSubCall::call_sync(
        api,
        "OCPP-SND",
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
        "OCPP-SND",
        "StatusNotification",
        v106::StatusNotification::Request(v106::StatusNotificationRequest {
            connector_id: 1,
            status: v106::ChargePointStatus::Available,
            error_code: v106::ChargePointErrorCode::NoError,
            info: Some("Tux-Evse booting report".to_string()),
            timestamp: Some(get_utc()),
            vendor_id: None,
            vendor_error_code: None,
        }),
    )?;

    AfbSubCall::call_sync(
        api,
        "OCPP-SND",
        "Heartbeat",
        v106::Heartbeat::Request(v106::HeartbeatRequest {}),
    )?;

    if tic > 0 {
        AfbTimer::new("tic-timer")
            .set_period(tic)
            .set_decount(0)
            .set_callback(Box::new(TimerCtx {
                apiv4: api.get_apiv4(),
                mgr,
            }))
            .start()?;
    }
    Ok(())
}

fn update_charger_status(
    mgr: &ManagerHandle,
    status: &OcppChargerStatus,
) -> Result<v106::StatusNotificationRequest, AfbError> {
    let mut error_code = v106::ChargePointErrorCode::NoError;

    let charger_status = match status {
        OcppChargerStatus::Charging => v106::ChargePointStatus::Charging,
        OcppChargerStatus::Reserved => v106::ChargePointStatus::Reserved,
        OcppChargerStatus::Unavailable => v106::ChargePointStatus::Unavailable,
        OcppChargerStatus::Available => v106::ChargePointStatus::Available,
        OcppChargerStatus::Finishing => v106::ChargePointStatus::Finishing,
        OcppChargerStatus::Preparing => v106::ChargePointStatus::Preparing,
        OcppChargerStatus::Error(err_code) => {
            error_code = match err_code {
                OcppErrorCode::ConnectorLockFailure => {
                    v106::ChargePointErrorCode::ConnectorLockFailure
                }
                OcppErrorCode::GroundFailure => v106::ChargePointErrorCode::GroundFailure,
                OcppErrorCode::HighTemperature => v106::ChargePointErrorCode::HighTemperature,
                OcppErrorCode::InternalError => v106::ChargePointErrorCode::InternalError,
                OcppErrorCode::NoError => v106::ChargePointErrorCode::NoError,
                OcppErrorCode::OtherError => v106::ChargePointErrorCode::OtherError,
                OcppErrorCode::OverCurrentFailure => v106::ChargePointErrorCode::OverCurrentFailure,
                OcppErrorCode::OverVoltage => v106::ChargePointErrorCode::OverVoltage,
                OcppErrorCode::PowerMeterFailure => v106::ChargePointErrorCode::PowerMeterFailure,
                OcppErrorCode::PowerSwitchFailure => v106::ChargePointErrorCode::PowerSwitchFailure,
                OcppErrorCode::ReaderFailure => v106::ChargePointErrorCode::ReaderFailure,
                OcppErrorCode::UnderVoltage => v106::ChargePointErrorCode::UnderVoltage,
                OcppErrorCode::WeakSignal => v106::ChargePointErrorCode::WeakSignal,
            };
            v106::ChargePointStatus::Faulted
        }
    };

    let query = v106::StatusNotificationRequest {
        connector_id: mgr.get_cid(),
        error_code: error_code, // IdToken, should this be a type?
        status: charger_status,
        info: None,
        timestamp: Some(get_utc()),
        vendor_id: None,
        vendor_error_code: None,
    };

    Ok(query)
}

// ref: https://www.ampcontrol.io/ocpp-guide/how-to-send-ocpp-meter-values-with-metervalues-req
fn engy_event_action(
    state: &EnergyState,
    mgr: &'static ManagerHandle,
) -> Result<v106::MeterValuesRequest, AfbError> {
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
        unit: Some(v106::UnitOfMeasure::W),
        measurand: Some(v106::Measurand::PowerActiveImport),
    };

    let session_value = v106::SampledValue {
        value: (state.session * 10).to_string(),
        location: None, // string
        context: None,  // string
        phase: None,    // default L1
        format: None,   // default Raw
        unit: Some(v106::UnitOfMeasure::Wh),
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

// async callback for pushing measures
AfbVerbRegister!(MeterValuesRsp, meter_values_response);
fn meter_values_response(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::MeterValues>(0)?;
    match data {
        v106::MeterValues::Response(response) => response,
        _ => return afb_error!("ocpp-metervalue-rsp", "invalid response type"),
    };
    afb_log_msg!(Debug, rqt, "MeterValues response accepted");
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct EngyStateRqtCtx {
    mgr: &'static ManagerHandle,
}
// this verb is only for testing purpose real measure should be send from engy event
AfbVerbRegister!(EngyStateRqt, engy_state_request, EngyStateRqtCtx);
fn engy_state_request(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut EngyStateRqtCtx,
) -> Result<(), AfbError> {
    let state = args.get::<&EnergyState>(0)?;

    let query = engy_event_action(state, ctx.mgr)?;
    afb_log_msg!(Debug, rqt, "MeterValues request: {:?}", query);

    AfbSubCall::call_async(
        rqt,
        "OCPP-SND",
        "MeterValues",
        v106::MeterValues::Request(query),
        Box::new(MeterValuesRsp {}),
    )?;

    AfbSubCall::call_sync(
        rqt,
        "OCPP-SND",
        "StatusNotification",
        v106::StatusNotification::Request(v106::StatusNotificationRequest {
            connector_id: 1,
            status: v106::ChargePointStatus::Charging,
            error_code: v106::ChargePointErrorCode::NoError,
            info: Some("Tux-Evse refresh report".to_string()),
            timestamp: Some(get_utc()),
            vendor_id: None,
            vendor_error_code: None,
        }),
    )?;

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
        "Heartbeat response nonce:{} time={}",
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
        "OCPP-SND",
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
        "OCPP-SND",
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
                "fail start:{:?}",
                response.id_tag_info.status
            )
        }
    };
    afb_log_msg!(Debug, rqt, "Transaction Start accepted tid:{}", tid);
    ctx.mgr.login(tid)?;
    rqt.reply(tid, 0);
    Ok(())
}

// Transaction stop async response callback
pub struct TransacStopRspCtx {
    pub mgr: &'static ManagerHandle,
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

    afb_log_msg!(Debug, rqt, "Transaction Stop accepted");
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
            ctx.mgr.check_active_session(false)?;
            let query = v106::StartTransactionRequest {
                connector_id: ctx.mgr.get_cid(),
                id_tag: tag.clone(),
                meter_start: 0,
                reservation_id: None,
                timestamp: get_utc(),
            };

            afb_log_msg!(Debug, rqt, "Transaction Start request: {:?}", &query);
            AfbSubCall::call_async(
                rqt,
                "OCPP-SND",
                "StartTransaction",
                v106::StartTransaction::Request(query),
                Box::new(TransacStartRspCtx { mgr: ctx.mgr }),
            )?;
        }
        OcppTransaction::Stop(meter) => {
            let tid = ctx.mgr.get_tid()?;
            ctx.mgr.check_active_session(true)?;
            let query = v106::StopTransactionRequest {
                id_tag: None,
                meter_stop: *meter,
                timestamp: get_utc(),
                reason: None,
                transaction_data: None,
                transaction_id: tid,
            };
            afb_log_msg!(Debug, rqt, "Transaction Stop request {:?}", &query);

            AfbSubCall::call_async(
                rqt,
                "OCPP-SND",
                "StopTransaction",
                v106::StopTransaction::Request(query),
                Box::new(TransacStopRspCtx { mgr: ctx.mgr }),
            )?;
        }
    }
    Ok(())
}

// StatusNotification async start response callback

AfbVerbRegister!(StatusNotificationRsp, status_notification_rsp);
fn status_notification_rsp(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::StatusNotification>(0)?;
    let _response = match data {
        v106::StatusNotification::Response(response) => response,
        _ => return afb_error!("ocpp-status-notification", "invalid response type"),
    };
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

// Authentication check id_tag on backend
struct StatusNotificationRqtCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(
    StatusNotificationRqt,
    status_notification_rqt,
    StatusNotificationRqtCtx
);
fn status_notification_rqt(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut StatusNotificationRqtCtx,
) -> Result<(), AfbError> {
    // move from binding to ocpp status
    let status = args.get::<&OcppChargerStatus>(0)?;
    ctx.mgr.set_status(&status)?;
    let query = update_charger_status(ctx.mgr, &status)?;

    afb_log_msg!(Debug, rqt, "Status Notification update{:?}", &query);
    AfbSubCall::call_async(
        rqt,
        "OCPP-SND",
        "StatusNotification",
        v106::StatusNotification::Request(query),
        Box::new(StatusNotificationRsp {}),
    )?;
    Ok(())
}

struct SubscribeData {
    mgr: &'static ManagerHandle,
}

AfbVerbRegister!(SubscribeCtrl, subscribe_callback, SubscribeData);
fn subscribe_callback(
    request: &AfbRequest,
    args: &AfbData,
    ctx: &mut SubscribeData,
) -> Result<(), AfbError> {
    let subcription = args.get::<bool>(0)?;
    ctx.mgr.subscribe(request, subcription)?;
    request.reply(AFB_NO_DATA, 0);
    Ok(())
}

pub(crate) fn register_frontend(api: &mut AfbApi, config: &BindingConfig) -> Result<(), AfbError> {
    let heartbeat_verb = AfbVerb::new("heartbeat")
        .set_callback(Box::new(HeartbeatRqt {}))
        .set_info("Request ping backend")
        .finalize()?;

    let authorize_verb = AfbVerb::new("authorize")
        .set_callback(Box::new(AuthorizeRqtCtx { mgr: config.mgr }))
        .set_info("Request tagid authorization from backend")
        .set_usage("idTag")
        .finalize()?;

    let transaction_verb = AfbVerb::new("transaction")
        .set_callback(Box::new(TransacRqtCtx { mgr: config.mgr }))
        .set_info("send start/stop transaction to backend")
        .set_usage("'idTag'")
        .finalize()?;

    let status_notification_verb = AfbVerb::new("status-notification")
        .set_callback(Box::new(StatusNotificationRqtCtx { mgr: config.mgr }))
        .set_info("Send status notification to backend")
        .set_sample("'Charging'")?
        .set_sample("'Available'")?
        .set_usage("ocpp-status")
        .finalize()?;

    let engy_state_verb = AfbVerb::new("push-measure")
        .set_callback(Box::new(EngyStateRqtCtx { mgr: config.mgr }))
        .set_info("mock engy state event")
        .finalize()?;

    let subscribe_verb = AfbVerb::new("subscribe")
        .set_callback(Box::new(SubscribeCtrl { mgr: config.mgr }))
        .set_info("subscribe auth-msg event")
        .set_usage("true|false")
        .finalize()?;

    let monitor_handler = AfbEvtHandler::new("monitor-evt")
        .set_pattern("monitor/disconnected")
        .set_callback(Box::new(MonitorEvtCtx {mgr: config.mgr }))
        .finalize()?;

    // register veb within API
    api.add_verb(authorize_verb);
    api.add_verb(transaction_verb);
    api.add_verb(status_notification_verb);
    api.add_verb(engy_state_verb);
    api.add_verb(heartbeat_verb);
    api.add_verb(subscribe_verb);
    api.add_evt_handler(monitor_handler);

    Ok(())
}
