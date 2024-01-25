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

use afbv4::prelude::*;
use serde::{Deserialize, Serialize};

AfbDataConverter!(ocpp_msg, OcppMsg);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OcppMsg {
    Initialized,
}

AfbDataConverter!(ocpp_transaction, OcppTransaction);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OcppTransaction {
    Start(String),
    Stop(i32),
}

AfbDataConverter!(ocpp_state, OcppState);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OcppState {
    pub connector_id: u32,
    pub tid: i32,
    pub authorized: bool,
}

impl OcppState {
    pub fn default() -> Self {
        OcppState {
            connector_id:0,
            tid: 0,
            authorized: false,
        }
    }
}

pub fn ocpp_registers() -> Result<(),AfbError> {
    ocpp_msg::register()?;
    ocpp_state::register()?;
    ocpp_transaction::register()?;
    Ok(())
}