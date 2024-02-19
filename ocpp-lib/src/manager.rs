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
use std::cell::{RefCell, RefMut};
use typesv4::prelude::*;

pub struct ManagerHandle {
    apiv4: AfbApiV4,
    event: &'static AfbEvent,
    data_set: RefCell<OcppState>,
    engy_api: &'static str,
    cid: u32,
}

impl ManagerHandle {
    pub fn new(
        apiv4: AfbApiV4,
        cid: u32,
        event: &'static AfbEvent,
        engy_api: &'static str,
    ) -> &'static mut Self {
        let handle = ManagerHandle {
            data_set: RefCell::new(OcppState::default()),
            engy_api,
            apiv4,
            event,
            cid,
        };

        // return a static handle to prevent Rust from complaining when moving/sharing it
        Box::leak(Box::new(handle))
    }

    #[track_caller]
    pub fn get_state(&self) -> Result<RefMut<'_, OcppState>, AfbError> {
        match self.data_set.try_borrow_mut() {
            Err(_) => return afb_error!("ocppmanager-update", "fail to access &mut data_set"),
            Ok(value) => Ok(value),
        }
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
                "Already running session tid:{}",
                data_set.tid
            );
        }
        Ok(())
    }

    pub fn get_cid(&self) -> u32 {
        self.cid
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
        afb_log_msg!(Debug, self.apiv4, "login tid:{} (EnergyAction::SUBSCRIBE)", data_set.tid);
        AfbSubCall::call_sync(self.apiv4, self.engy_api, "state", EnergyAction::SUBSCRIBE)?;
        Ok(())
    }

    pub fn logout(&self) -> Result<(), AfbError> {
        let mut data_set = self.get_state()?;
        data_set.tid = 0;
        AfbSubCall::call_sync(
            self.apiv4,
            self.engy_api,
            "state",
            EnergyAction::UNSUBSCRIBE,
        )?;
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
}
