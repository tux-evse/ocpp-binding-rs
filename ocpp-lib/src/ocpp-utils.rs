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
use afbv4::prelude::*;
use chrono::{DateTime, Utc};


pub fn now_to_duration(date: DateTime<Utc>) -> Result <std::time::Duration, AfbError> {
    let now= Utc::now();
    let offset= date - now;
    match offset.to_std() {
        Ok(value) => {Ok(value)},
        Err(_) => {afb_error!("chronos-duration-fail", "impossible conversion")}
    }
}


