/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Ocppor: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 */

//use crate::prelude::*;
use crate::prelude::*;
use afbv4::prelude::*;
use std::sync::{Mutex, MutexGuard};
use typesv4::prelude::*;

pub struct ManagerHandle {
    event: &'static AfbEvent,
    data_set: Mutex<OcppState>,
    cid: u32,
}

impl ManagerHandle {
    pub fn new(
        cid: u32,
        event: &'static AfbEvent,
    ) -> &'static mut Self {
        let handle = ManagerHandle {
            data_set: Mutex::new(OcppState::default()),
            event,
            cid,
        };

        // return a static handle to prevent Rust from complaining when moving/sharing it
        Box::leak(Box::new(handle))
    }

    #[track_caller]
    pub fn get_state(&self) -> Result<MutexGuard<'_, OcppState>, AfbError> {
        let guard = self.data_set.lock().unwrap();
        Ok(guard)
    }

    pub fn subscribe(&self, request: &AfbRequest, subscription: bool) -> Result<(), AfbError> {
        if subscription {
            self.event.subscribe(request)?;
        } else {
            self.event.unsubscribe(request)?;
        }
        Ok(())
    }

    pub fn check_active_session(&self, status: bool) -> Result<(), AfbError> {
        let data_set = self.get_state()?;
        if status && data_set.tid == 0 {
            return afb_error!("ocpp-active-session", "No active session tid");
        }
        if !status && data_set.tid != 0 {
            return afb_error!(
                "ocpp-running-session",
                "Running session tid:{} already running",
                data_set.tid
            );
        }
        Ok(())
    }

    pub fn get_cid(&self) -> u32 {
        self.cid
    }

    pub fn set_status(&self, status: &OcppChargerStatus) -> Result<(), AfbError> {
        let mut data_set = self.get_state()?;
        data_set.status= status.clone();
        Ok(())
    }

    pub fn get_status(&self) -> Result<OcppChargerStatus, AfbError> {
        let data_set = self.get_state()?;
        Ok(data_set.status.clone())
    }

    pub fn get_tid(&self) -> Result<i32, AfbError> {
        let data_set = self.get_state()?;
        Ok(data_set.tid)
    }

    pub fn authorized(&self, authorized: bool) -> Result<(), AfbError> {
        let mut data_set = self.get_state()?;
        self.event.push(OcppMsg::Authorized(authorized));
        data_set.authorized = authorized;
        Ok(())
    }

    pub fn login(&self, tid: i32) -> Result<(), AfbError> {
        let mut data_set = self.get_state()?;

        data_set.tid = tid;
        Ok(())
    }

    pub fn logout(&self) -> Result<(), AfbError> {
        let mut data_set = self.get_state()?;
        data_set.tid = 0;
        Ok(())
    }

    pub fn reset(&self) -> Result<(), AfbError> {
        self.event.push(OcppMsg::Reset);
        self.logout()?;
        Ok(())
    }

    pub fn reserv_now(
        &self,
        request: ReservationSession,
    ) -> Result<v106::ReservationStatus, AfbError> {
        let mut data_set = self.get_state()?;

        let response = match &data_set.reservation {
            Some(_value) => v106::ReservationStatus::Occupied,
            None => {
                self.event.push(OcppMsg::Reservation(request.clone()));
                data_set.reservation = Some(request);
                v106::ReservationStatus::Accepted
            }
        };
        Ok(response)
    }

    pub fn reserv_cancel(&self, rid: i32) -> Result<v106::CancelReservationStatus, AfbError> {
        let mut data_set = self.get_state()?;

        let response = match &data_set.reservation {
            None => v106::CancelReservationStatus::Rejected,
            Some(resa) => {
                if rid != resa.id {
                    v106::CancelReservationStatus::Rejected
                } else {
                    let mut cancel = resa.clone();
                    cancel.status = ReservationStatus::Cancel;
                    self.event.push(OcppMsg::Reservation(cancel));
                    data_set.reservation = None;
                    v106::CancelReservationStatus::Accepted
                }
            }
        };
        Ok(response)
    }

    pub fn set_limit(&self, limit: PowerLimit) -> Result<v106::ChargingProfileStatus, AfbError> {
        let data_set = self.get_state()?;

        let response = if limit.tid != data_set.tid {
            v106::ChargingProfileStatus::Rejected
        } else {
            self.event.push(OcppMsg::PowerLimit(limit));
            v106::ChargingProfileStatus::Accepted
        };
        Ok(response)
    }


    //Draft RMU: send remote stop request from OCPP to ChargingMgr
    // ?? RemoteStopTransaction ou remoteStopTransaction
    pub fn remote_stop_transaction(&self, remote_stop_transaction_id: i32) -> Result<(), AfbError> {
        let mut data_set = self.get_state()?;

        if data_set.tid != remote_stop_transaction_id {
            return afb_error! ("ocpp-remote-stop-mgr", "invalid transaction id expect:{} get:{}", data_set.tid, remote_stop_transaction_id)
        }

	    self.event.push(OcppMsg::Transaction(false, remote_stop_transaction_id as u32));
        data_set.tid=0;
        Ok(())
    }

    //pub fn authorized(&self, authorized: bool) -> Result<(), AfbError> {
    //    let mut data_set = self.get_state()?;
    //    self.event.push(OcppMsg::Authorized(authorized));
    //    data_set.authorized = authorized;
    //    Ok(())
    //}


}
