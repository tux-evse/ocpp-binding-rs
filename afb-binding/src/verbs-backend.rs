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
use std::time::Duration;
use typesv4::prelude::*;

// Generic callback when ocpp response should be ignored
AfbVerbRegister!(IgnoreOcppBackendRsp, ignore_backend_rsp);
fn ignore_backend_rsp(rqt: &AfbRequest, _args: &AfbData) -> Result<(), AfbError> {
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

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
        v106::Heartbeat::Request(_data) => {
            ctx.count = ctx.count + 1;
            afb_log_msg!(Debug, rqt, "Heartbeat count:{}", ctx.count);
            let data = v106::Heartbeat::Response(v106::HeartbeatResponse {
                current_time: get_utc(),
            });
            rqt.reply(data, 0);
        }
        _ => {
            afb_log_msg!(Warning, rqt, "Unsupported reset request");
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

struct CancelReservationCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(
    CancelReservationVerb,
    cancel_notification_cb,
    CancelReservationCtx
);
fn cancel_notification_cb(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut CancelReservationCtx,
) -> Result<(), AfbError> {
    let data = args.get::<&v106::CancelReservation>(0)?;
    match data {
        v106::CancelReservation::Request(value) => {
            afb_log_msg!(Debug, rqt, "Backend cancel reservation{:?}", value);
            let status = ctx.mgr.reserv_cancel(value.reservation_id)?;
            let response = v106::CancelReservationResponse { status };
            rqt.reply(v106::CancelReservation::Response(response), 0);
        }
        _ => {
            afb_log_msg!(Warning, rqt, "Unsupported reset request");
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

// 6.37. ReserveNow.req
struct ReserveNowCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(ReserveNowVerb, reverve_now_cb, ReserveNowCtx);
fn reverve_now_cb(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut ReserveNowCtx,
) -> Result<(), AfbError> {
    let data = args.get::<&v106::ReserveNow>(0)?;
    match data {
        v106::ReserveNow::Request(value) => {
            afb_log_msg!(Debug, rqt, "Backend reserve now {:?}", value);

            let reservation = ReservationSession {
                id: value.reservation_id,
                tagid: value.id_tag.clone(),
                start: Duration::new(0, 0),
                stop: now_to_duration(value.expiry_date)?,
                status: ReservationStatus::Pending,
            };

            let status = ctx.mgr.reserv_now(reservation)?;
            let response = v106::ReserveNowResponse { status };
            rqt.reply(v106::ReserveNow::Response(response), 0);
        }
        _ => {
            afb_log_msg!(Warning, rqt, "Unsupported reset request");
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

AfbVerbRegister!(ChangeAvailabilityVerb, change_availability_cb);
fn change_availability_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::ChangeAvailability>(0)?;
    match data {
        v106::ChangeAvailability::Request(value) => {
            afb_log_msg!(Debug, rqt, "Backend cancel reservation{:?}", value);
            let _id = value.connector_id;
            match &value.kind {
                v106::AvailabilityType::Operative => {}
                v106::AvailabilityType::Inoperative => {}
            }

            // Fulup TBD Do something status= Accepted|Rejected|Scheduled
            let response = v106::ChangeAvailabilityResponse {
                status: v106::AvailabilityStatus::Accepted,
            };
            rqt.reply(v106::ChangeAvailability::Response(response), 0);
        }
        _ => {
            afb_log_msg!(Warning, rqt, "Unsupported reset request");
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

struct ResetVerbCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(ResetVerb, reset_cb, ResetVerbCtx);


fn reset_cb(
        rqt: &AfbRequest, 
        args: &AfbData, 
        ctx: &mut ResetVerb
) -> Result<(), AfbError> {
    let data = args.get::<&v106::Reset>(0)?;
    match data {
        v106::Reset::Request(reset) => {
            afb_log_msg!(Debug, rqt, "Backend reset {:?}", reset);
            let status = match reset.kind {
                v106::ResetRequestStatus::Hard => {
                    // should reboot hardware
                    afb_log_msg!(Warning, rqt, "Hard reset (hardware reboot) ignored");
                    v106::ResetResponseStatus::Rejected
                }
                v106::ResetRequestStatus::Soft => {
                    ctx.mgr.reset()?;
                    v106::ResetResponseStatus::Accepted
                }
            };

            let response = v106::ResetResponse { status };
            rqt.reply(v106::Reset::Response(response), 0);
        }
        _ => {
            afb_log_msg!(Warning, rqt, "Unsupported reset request");
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

struct SetChargingProfileCtx {
    mgr: &'static ManagerHandle,
}
// 6.43. SetChargingProfile.req
// https://www.ampcontrol.io/ocpp-guide/how-to-use-smart-charging-with-ocpp
AfbVerbRegister!(
    SetChargingProfileVerb,
    set_charging_profile_cb,
    SetChargingProfileCtx
);
fn set_charging_profile_cb(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut SetChargingProfileCtx,
) -> Result<(), AfbError> {
    let data = args.get::<&v106::SetChargingProfile>(0)?;
    match data {
        v106::SetChargingProfile::Request(value) => {
            let target_tid = match value.cs_charging_profiles.transaction_id {
                Some(value) => value,
                None => -1,
            };

            // to avoid log mess we try to kill any invalid transaction
            let session_tid = ctx.mgr.get_tid()?;
            if target_tid != session_tid {
                afb_log_msg!(
                    Notice,
                    rqt,
                    "Ignored set-charging-profile backend_tid:{} != session_tid:{}",
                    target_tid,
                    session_tid
                );
                let status = v106::ChargingProfileStatus::Rejected;
                let response = v106::SetChargingProfileResponse { status };
                rqt.reply(v106::SetChargingProfile::Response(response), 0);

                // let force stop invalid transaction ID
                let query = v106::StopTransactionRequest {
                    id_tag: None,
                    meter_stop: 0,
                    timestamp: get_utc(),
                    reason: None,
                    transaction_data: None,
                    transaction_id: target_tid,
                };

                AfbSubCall::call_sync(
                    rqt.get_api(),
                    "OCPP-SND",
                    "StopTransaction",
                    v106::StopTransaction::Request(query),
                )?;
                return Ok(());
            }

            afb_log_msg!(
                Debug,
                rqt,
                "Backend set-charging-profile accepted {:?}",
                value
            );
            let duration = value
                .cs_charging_profiles
                .charging_schedule
                .duration
                .unwrap();
            let limit = value
                .cs_charging_profiles
                .charging_schedule
                .charging_schedule_period[0]
                .limit;

            let limit = PowerLimit {
                tid: target_tid,
                imax: (limit * 100.0).round() as i32,
                duration: duration as u32,
            };

            let status = ctx.mgr.set_limit(limit)?;
            let response = v106::SetChargingProfileResponse { status };
            rqt.reply(v106::SetChargingProfile::Response(response), 0);
        }
        _ => {
            afb_log_msg!(Warning, rqt, "Unsupported reset request");
            rqt.reply(AFB_NO_DATA, 0); // if returning error bia will cut connection
        }
    }

    Ok(())
}

// Fulup Verbs TDB
// -----------------
// 6.11. ClearCache.req
// 6.13. ClearSetChargingProfile.req
// 6.15. DataTransfer.req
// 6.21. GetCompositeSchedule.req
// 6.23. GetConfiguration.req
// 6.25. GetDiagnostics.req
// 6.27. GetLocalListVersion.req
// 6.33. RemoteStartTransaction.req
// 6.39. Reset.req
// 6.41. SendLocalList.req
// 6.51. TriggerMessage.req
// 6.53. UnlockConnector.req
// 6.55. UpdateFirmware.req




// 6.33. RemoteStopTransaction.req   RMU
struct RemoteStopTransactionCtx {
    mgr: &'static ocpp::manager::ManagerHandle,
}
AfbVerbRegister!(RemoteStopTransaction, remote_stop_transaction_cb, RemoteStopTransactionCtx);



fn remote_stop_transaction_cb(
        rqt: &AfbRequest, 
        args: &AfbData, 
        ctx: &mut RemoteStopTransactionCtx
) -> Result<(), AfbError> {
    let data = args.get::<&v106::RemoteStopTransaction>(0)?;
    match data {
        v106::RemoteStopTransaction::Request(value) => {
            afb_log_msg!(Debug, rqt, "Backend Remote Stop Transaction req {:?}", value);
            //let status = ctx.mgr.remote_stop_transaction(false)?;
            //let response = v106::RemoteStopTransactionResponse { status };
            //rqt.reply(v106::RemoteStopTransaction::Response(response), 0);
            ctx.mgr.remote_stop_transaction(false)?;
        }
        _ => {
            afb_log_msg!(Warning, rqt, "Unsupported remote stop request request");
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}





pub(crate) fn register_backend(api: &mut AfbApi, config: &BindingConfig) -> Result<(), AfbError> {
    let cancel_resa = AfbVerb::new("CancelReservation")
        .set_callback(Box::new(CancelReservationVerb { mgr: config.mgr }))
        .set_info("backend cancel reservation")
        .finalize()?;

    let reserve_now = AfbVerb::new("ReserveNow")
        .set_callback(Box::new(ReserveNowCtx { mgr: config.mgr }))
        .set_info("backend frontend reservation")
        .finalize()?;

    let reset = AfbVerb::new("Reset")
        .set_callback(Box::new(ResetVerbCtx { mgr: config.mgr }))
        .set_info("backend request frontend reset")
        .finalize()?;

    let remoteStopTransaction = AfbVerb::new("RemoteStopTransaction")
        .set_callback(Box::new(RemoteStopTransactionCtx { mgr: config.mgr }))
	.set_info("backend request to stop transaction")
        .finalize()?;

    let setprofile = AfbVerb::new("SetChargingProfile")
        .set_callback(Box::new(SetChargingProfileCtx { mgr: config.mgr }))
        .set_info("backend request SetChargingProfile")
        .finalize()?;

    api.add_verb(cancel_resa);
    api.add_verb(reserve_now);
    api.add_verb(setprofile);
    api.add_verb(reset);
    api.add_verb(remoteStopTransaction);

    Ok(())
}
