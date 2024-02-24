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
use  std::time::Duration;

AfbDataConverter!(meter_tag_set, MeterTagSet);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum MeterTagSet {
    Current,
    Tension,
    Power,
    OverCurrent,
    AvailCurrent,
    Energy,
    #[default]
    Unset,
}

AfbDataConverter!(config_data_set, EngyConfSet);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EngyConfSet {
    pub pmax: i32,
    pub imax: i32,
}

// all meter in 00.A value
AfbDataConverter!(meter_data_set, MeterDataSet);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MeterDataSet {
    #[serde(skip)]
    pub start: i32,
    #[serde(skip)]
    pub variation: i32,
    #[serde(skip)]
    pub updated: bool,
    pub tag: MeterTagSet,
    pub total: i32,
    pub l1: i32,
    pub l2: i32,
    pub l3: i32,
}

impl MeterDataSet {
    pub fn default(tag: MeterTagSet) -> Self {
        let variation= match tag {
            MeterTagSet::AvailCurrent => 0,
            MeterTagSet::OverCurrent => 0,
            _ => 1,
        };

        MeterDataSet {
            tag: tag,
            variation: variation,
            start: 0,
            updated: false,
            total: 0,
            l1: 0,
            l2: 0,
            l3: 0,
        }
    }

    // update data_set and set updated flag when total changes.
    pub fn update(&mut self, phase: usize, meter: f64) -> Result<(), AfbError> {
        let value = (meter * 100.0).round() as i32;
        let variation = value*self.variation/100;
        match phase {
            0 => {
                let value = value - self.start;
                if self.total - variation < value || self.total + variation > value
                {
                    self.total = value;
                    self.updated = true;
                }
            }
            1 => {
                if self.l1 - variation < value || self.l1 + variation > value
                {
                    self.l1 = value;
                }
            }
            2 => {
                if self.l2 - variation < value || self.l2 + variation > value
                {
                    self.l2 = value;
                }
            }
            3 => {
                if self.l3 - variation < value || self.l3 + variation > value
                {
                    self.l3 = value;
                }
            }
            _ => return afb_error!("data-set-update", "invalid phase:{}", phase),
        }
        Ok(())
}
}

AfbDataConverter!(energy_actions, EnergyAction);
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase", tag = "action")]
pub enum EnergyAction {
    #[default]
    READ,
    SUBSCRIBE,
    UNSUBSCRIBE,
    RESET,
    INFO,
}

AfbDataConverter!(energy_state, EnergyState);
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EnergyState {
    #[serde(skip)]
    pub subscription_max: i32,
    #[serde(skip)]
    pub umax: i32,
    #[serde(skip)]
    pub imax: i32,
    #[serde(skip)]
    pub pmax: i32,
    pub volts: i32,
    // public data
    pub timestamp: Duration,
    pub session: i32,
    pub total: i32,
    pub current: i32,
    pub tension: i32,
    pub power: i32,
}

impl EnergyState {
    pub fn default(imax: i32, pmax: i32,umax: i32) -> Self {
        EnergyState {
            subscription_max: 0,
            imax: imax*100,
            pmax: pmax*100,
            umax: umax*100,
            session: 0,
            total: 0,
            volts: 0,
            current: 0,
            tension: 0,
            power: 0,
            timestamp: Duration::new(0,0),
        }
    }
}

pub fn engy_registers() -> Result<(), AfbError> {
    meter_data_set::register()?;
    config_data_set::register()?;
    energy_actions::register()?;
    energy_state::register()?;
    meter_tag_set::register()?;
    Ok(())
}
