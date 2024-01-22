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
use serde::{Deserialize, Serialize};
use std::time::Duration;

AfbDataConverter!(error_state, ErrorState);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ErrorState {
    ErrE,
    ErrDf,
    ErrRelay,
    ErrRdc,
    ErrOverCurrent,
    ErrPermanent,
    ErrVentilation,
}

AfbDataConverter!(power_request, PowerRequest);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PowerRequest {
    Start,
    Charging(u32),
    Stop(i32),
    Idle,
}

AfbDataConverter!(plug_state, PlugState);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PlugState {
    PlugIn,
    Lock,
    Error,
    PlugOut,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum IsoState {
    Iso20,
    Iso2,
    Iec,
    Unset,
}

AfbDataConverter!(charging_event, ChargingMsg);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ChargingMsg {
    Plugged(PlugState),
    Power(PowerRequest),
    Iso(IsoState),
    Auth(AuthMsg),
    State(ChargingState),
    Reservation(ReservationStatus)
}

AfbDataConverter!(reservation_state, ReservationState);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ReservationState {
    pub id: i64,
    pub start: Duration,
    pub stop: Duration,
}

AfbDataConverter!(charging_state, ChargingState);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ChargingState {
    #[serde(skip)]
    pub updated: bool,
    #[serde(skip)]
    pub reservation: Option<ReservationState>,
    pub imax: u32,
    pub pmax: u32,
    pub plugged: PlugState,
    pub power: PowerRequest,
    pub iso: IsoState,
    pub auth: AuthMsg,

}

impl ChargingState {
    pub fn default() -> Self {
        ChargingState {
            updated: false,
            imax: 32, // Fulup TBD should comme from energy mgr
            pmax: 22,
            plugged: PlugState::Unknown,
            power: PowerRequest::Idle,
            iso: IsoState::Unset,
            auth: AuthMsg::Idle,
            reservation: None,
        }
    }
}

AfbDataConverter!(charging_actions, ChargingAction);
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase", tag = "action")]
pub enum ChargingAction {
    #[default]
    READ,
    SUBSCRIBE,
    UNSUBSCRIBE,
}
pub enum ReservationAction {
    NOW,
    DELAY,
    CANCEL,
}

AfbDataConverter!(charging_msg, ReservationStatus);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ReservationStatus {
    Accepted,
    Refused,
    Pending,
    Cancel,
    Request,
}

AfbDataConverter!(reservation_session, ReservationSession);
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ReservationSession {
    pub id: i64,
    pub tagid:String,
    pub start: Duration,
    pub stop:  Duration,
    pub status: ReservationStatus,
}

pub fn chmgr_registers() -> Result<(), AfbError> {
    charging_actions::register()?;
    plug_state::register()?;
    charging_state::register()?;
    error_state::register()?;
    power_request::register()?;
    charging_event::register()?;
    reservation_session::register()?;
    reservation_state::register()?;

    Ok(())
}
