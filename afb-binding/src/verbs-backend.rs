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

use chrono::Utc;
use afbv4::prelude::*;
use libocpp::{prelude::*, v106::AvailabilityType};

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
                current_time: Utc::now(),
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

AfbVerbRegister!(CancelReservationVerb, cancel_notification_cb);
fn cancel_notification_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let data = args.get::<&v106::CancelReservation>(0)?;
    match data {
        v106::CancelReservation::Request(value) => {
            afb_log_msg!(Debug, rqt, "Backend cancel reservation{:?}", value);
            let id = value.reservation_id;

            // Fulup TBD Do something status: Accepted|Rejected
            let response= v106::CancelReservationResponse {status:v106::CancelReservationStatus::Accepted};
            rqt.reply(v106::CancelReservation::Response(response), 0);
        }
        v106::CancelReservation::Response(value) => {
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
                    AvailabilityType::Operative => {},
                    AvailabilityType::Inoperative => {},
            }

            // Fulup TBD Do something status= Accepted|Rejected|Scheduled
            let response= v106::ChangeAvailabilityResponse {status:v106::AvailabilityStatus::Accepted};
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
            let response= v106::ResetResponse {status:v106::ResetResponseStatus::Accepted};
            rqt.reply(v106::Reset::Response(response), 0);
        }
        v106::Reset::Response(value) => {
            afb_log_msg!(Warning, rqt, "Ignore reset data {:?}", value);
            rqt.reply(AFB_NO_DATA, -1);
        }
    }
    Ok(())
}

// Fulup Verbs TDB
// -----------------
// 6.11. ClearCache.req
// 6.13. ClearChargingProfile.req
// 6.15. DataTransfer.req
// 6.21. GetCompositeSchedule.req
// 6.23. GetConfiguration.req
// 6.25. GetDiagnostics.req
// 6.27. GetLocalListVersion.req
// 6.33. RemoteStartTransaction.req
// 6.37. ReserveNow.req
// 6.39. Reset.req
// 6.41. SendLocalList.req
// 6.43. SetChargingProfile.req
// 6.51. TriggerMessage.req
// 6.53. UnlockConnector.req
// 6.55. UpdateFirmware.req

pub(crate) fn register_backend(api: &mut AfbApi) -> Result<(), AfbError> {
    let newstate = AfbVerb::new("CancelReservation")
        .set_callback(Box::new(CancelReservationVerb {}))
        .set_info("backend cancel reservation")
        .finalize()?;

    let reset = AfbVerb::new("Reset")
        .set_callback(Box::new(ResetVerb {}))
        .set_info("backend request charger reset")
        .finalize()?;

    api.add_verb(newstate);
    api.add_verb(reset);

    Ok(())
}
