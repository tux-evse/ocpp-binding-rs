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
use afbv4::prelude::*;
use std::cell::{RefCell, RefMut};
use typesv4::prelude::*;

pub struct ManagerHandle {
    data_set: RefCell<OcppState>,
    engy_api: &'static str,
    cid: u32,
    apiv4: AfbApiV4,
}

impl ManagerHandle {
    pub fn new(
        cid: u32,
        event: &'static AfbEvent,
         engy_api: &'static str,
    ) -> &'static mut Self {
        let handle = ManagerHandle {
            data_set: RefCell::new(OcppState::default()),
            engy_api,
            apiv4: event.get_apiv4(),
            cid,
        };

        // return a static handle to prevent Rust from complaining when moving/sharing it
        Box::leak(Box::new(handle))
    }

    #[track_caller]
    pub fn get_state(&self) -> Result<RefMut<'_, OcppState>, AfbError> {
        match self.data_set.try_borrow_mut() {
            Err(_) => return afb_error!("charging-manager-update", "fail to access &mut data_set"),
            Ok(value) => Ok(value),
        }
    }

    pub fn check_active_session(&self, status: bool) -> Result<(), AfbError> {
        let data_set = self.get_state()?;
        if status && data_set.tid == 0 {
            return afb_error!("ocpp-active-session", "No active session tid");
        }
        if !status && data_set.tid != 0 {
            return afb_error!(
                "ocpp-active-session",
                "Active session running tid:{}",
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
        data_set.authorized = authorized;
        Ok(())
    }

    pub fn login(&self, tid: i32) -> Result<(), AfbError> {
        let mut data_set = self.get_state()?;
        data_set.tid = tid;
        AfbSubCall::call_sync(self.apiv4, self.engy_api, "state", EnergyAction::SUBSCRIBE)?;
        Ok(())
    }

    pub fn logout(&self) -> Result<(), AfbError> {
        let mut data_set = self.get_state()?;
        data_set.tid = 0;
        AfbSubCall::call_sync(self.apiv4, self.engy_api, "state", EnergyAction::UNSUBSCRIBE)?;
        Ok(())
    }
}
