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
use serde::{Deserialize, Serialize};

AfbDataConverter!(auth_msg, AuthMsg);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AuthMsg {
    Done,
    Fail,
    Pending,
    Idle,
}

AfbDataConverter!(auth_state, AuthState);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct AuthState {
    pub auth: AuthMsg,
    pub tagid:String,
    pub imax:u32,
    pub pmax: u32,
}

impl AuthState {
    pub fn default() -> Self {
        AuthState {
            auth: AuthMsg::Idle,
            imax: 0,
            pmax: 0,
            tagid: String::new(),
        }
    }
}

AfbDataConverter!(auth_actions, AuthAction);
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase", tag = "action")]
pub enum AuthAction {
    #[default]
    READ,
    SUBSCRIBE,
    UNSUBSCRIBE,
}


pub fn auth_registers() -> Result<(),AfbError> {
    auth_msg::register()?;
    auth_state::register()?;
    auth_actions::register()?;
    Ok(())
}