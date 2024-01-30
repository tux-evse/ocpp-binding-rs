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
        v106::Heartbeat::Response(data) => {
            afb_log_msg!(Warning, rqt, "Ignore heartbeat data {:?}", data);
            rqt.reply(AFB_NO_DATA, 0);
        }
    }
    Ok(())
}

struct CancelReservationCtx {
    chmgr_api: &'static str,
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

            let reservation = ReservationSession {
                id: value.reservation_id,
                tagid: String::new(),
                start: Duration::new(0, 0),
                stop: Duration::new(0, 0),
                status: ReservationStatus::Cancel,
            };

            let status = match AfbSubCall::call_sync(rqt, ctx.chmgr_api, "reserve", reservation) {
                Ok(_value) => v106::CancelReservationStatus::Accepted,
                Err(_error) => v106::CancelReservationStatus::Rejected,
            };

            let response = v106::CancelReservationResponse { status };
            rqt.reply(v106::CancelReservation::Response(response), 0);
        }
        v106::CancelReservation::Response(value) => {
            afb_log_msg!(Warning, rqt, "Ignore status_notification data {:?}", value);
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

// 6.37. ReserveNow.req
struct ReserveNowCtx {
    chmgr_api: &'static str,
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

            let status = match AfbSubCall::call_sync(rqt, ctx.chmgr_api, "reserve", reservation) {
                Ok(_value) => v106::ReservationStatus::Accepted,
                Err(_error) => v106::ReservationStatus::Rejected,
            };

            let response = v106::ReserveNowResponse { status };
            rqt.reply(v106::ReserveNow::Response(response), 0);
        }
        v106::ReserveNow::Response(value) => {
            afb_log_msg!(Warning, rqt, "Ignore status_notification data {:?}", value);
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
        v106::ChangeAvailability::Response(value) => {
            afb_log_msg!(Warning, rqt, "Ignore status_notification data {:?}", value);
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

struct ResetVerbCtx {
    mgr: &'static ManagerHandle,
}
AfbVerbRegister!(ResetVerb, reset_cb, ResetVerbCtx);
fn reset_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut ResetVerb) -> Result<(), AfbError> {
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

// 6.43. SetChargingProfile.req
// https://www.ampcontrol.io/ocpp-guide/how-to-use-smart-charging-with-ocpp
AfbVerbRegister!(SetChargingProfileVerb, set_charging_profile_cb);
fn set_charging_profile_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let _data = args.get::<&v106::SetChargingProfile>(0)?;
    // SetChargingProfile: {
    //     "connectorId": 1,
    //     "csChargingProfiles": {
    //         "chargingProfileId": 1,
    //         "transactionId": 21126266,
    //         "stackLevel": 99,
    //         "chargingProfilePurpose": "TxProfile",
    //         "chargingProfileKind": "Absolute",
    //         "chargingSchedule": {
    //             "duration": 3600,
    //             "startSchedule": "2024-01-29T18:29:41Z",
    //             "chargingRateUnit": "A",
    //             "chargingSchedulePeriod": [
    //                 {
    //                     "startPeriod": 0,
    //                     "limit": 3.1754264805429417,
    //                     "numberPhases": 3
    //                 }
    //             ]
    //         }
    //     }
    // }
    let response = v106::SetChargingProfileResponse {
        status: v106::ChargingProfileStatus::NotSupported,
    };
    rqt.reply(v106::SetChargingProfile::Response(response), 0);
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

pub(crate) fn register_backend(api: &mut AfbApi, config: &BindingConfig) -> Result<(), AfbError> {
    let cancel_resa = AfbVerb::new("CancelReservation")
        .set_callback(Box::new(CancelReservationVerb {
            chmgr_api: config.chmgr_api,
        }))
        .set_info("backend cancel reservation")
        .finalize()?;

    let reserve_now = AfbVerb::new("ReserveNow")
        .set_callback(Box::new(ReserveNowCtx {
            chmgr_api: config.chmgr_api,
        }))
        .set_info("backend frontend reservation")
        .finalize()?;

    let reset = AfbVerb::new("Reset")
        .set_callback(Box::new(ResetVerbCtx { mgr: config.mgr }))
        .set_info("backend request frontend reset")
        .finalize()?;

    let setprofile = AfbVerb::new("SetChargingProfile")
        .set_callback(Box::new(SetChargingProfileVerb {}))
        .set_info("backend request SetChargingProfile")
        .finalize()?;

    api.add_verb(cancel_resa);
    api.add_verb(reserve_now);
    api.add_verb(setprofile);
    api.add_verb(reset);

    Ok(())
}
