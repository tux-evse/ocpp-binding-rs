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
use std::time::Duration;


AfbDataConverter!(ocpp_msg, OcppReservedStatus);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OcppReservedStatus {
    Accepted,
    Refused,
    Unset,
}

AfbDataConverter!(ocpp_state, OcppReserved);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OcppReserved {
    pub id: i64,
    pub tagid:String,
    pub start: Duration,
    pub stop:  Duration,
    pub status: OcppReservedStatus,
}


pub fn ocpp_registers() -> Result<(),AfbError> {
    ocpp_msg::register()?;
    Ok(())
}