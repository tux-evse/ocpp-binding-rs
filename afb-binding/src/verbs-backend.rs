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

AfbVerbRegister!(ResetVerb, reset_cb);
fn reset_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::Reset>(0)?;
    match data {
        v106::Reset::Request(value) => {
            afb_log_msg!(Debug, rqt, "Backend reset {:?}", value);

            // Fulup TBD Do something status= Accepted|Rejected|Scheduled
            let response = v106::ResetResponse {
                status: v106::ResetResponseStatus::Accepted,
            };
            rqt.reply(v106::Reset::Response(response), 0);
        }
        v106::Reset::Response(value) => {
            afb_log_msg!(Warning, rqt, "Ignore reset data {:?}", value);
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

// https://www.ampcontrol.io/ocpp-guide/how-to-use-smart-charging-with-ocpp
// 6.43. SetChargingProfile.req
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
AfbVerbRegister!(SetChargingProfileVerb, set_charging_profile_cb);
fn set_charging_profile_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    afb_log_msg! (Debug, rqt, "received  SetChargingProfile");
    let data = args.get::<&v106::SetChargingProfile>(0)?;
    // match data {
    //     v106::SetChargingProfile::Request(value) => {
    //         afb_log_msg!(Debug, rqt, "Backend cancel reservation{:?}", value);

    //         let _id = value.connector_id;
    //         let profile = value.cs_charging_profiles;

    //         // check stack is lower ignore value
    //         // profile.stack_level

    //         match profile.charging_profile_purpose {
    //             ChargingProfilePurposeType::ChargePointMaxProfile => {}
    //             ChargingProfilePurposeType::TxDefaultProfile => {}
    //             ChargingProfilePurposeType::TxProfile => {}
    //         }

    //         match profile.chargingProfileKind {
    //             ChargingProfileKindType::Absolute => {}
    //             ChargingProfileKindType::Recurring => {}
    //             ChargingProfileKindType::Recurring => {}
    //         }

    //         profile.chargingSchedule {
    //             //          pub duration: Option<i32>,
    //             // /// Optional. Starting point of an absolute schedule. If absent the schedule will be relative to start of charging.
    //             // #[serde(skip_serializing_if = "Option::is_none")]
    //             // pub start_schedule: Option<DateTime<Utc>>,
    //             // /// Required. The unit of measure Limit is expressed in.
    //             // pub charging_rate_unit: ChargingRateUnitType,
    //             // /// Required. List of ChargingSchedulePeriod elements defining maximum power or current usage over time. The startSchedule of the first ChargingSchedulePeriod SHALL always be 0.
    //             // pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    //             // /// Optional. Minimum charging rate supported by the electric vehicle. The unit of measure is defined by the chargingRateUnit. This parameter is intended to be used by a local smart charging algorithm to optimize the power allocation for in the case a charging process is inefficient at lower charging rates. Accepts at most one digit fraction (e.g. 8.1)
    //             // #[serde(skip_serializing_if = "Option::is_none")]
    //             // pub min_charging_rate: Option<f32>,
    //         }

    //         // ignore recurrencyKind
    //         // validTo

    //         //ctx.mgr.set_cpid= profile.charging_profile_id;
    //         if let Some(value) = profile.transaction_id {
    //             // check transaction id on mgr
    //         }

            // Fulup TBD Do something status= Accepted|Rejected|Scheduled
            let response = v106::SetChargingProfileResponse {
                status: v106::ChargingProfileStatus::Accepted,
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
        .set_callback(Box::new(ResetVerb {}))
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
