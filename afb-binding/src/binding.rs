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
use ocpp::prelude::*;
use typesv4::prelude::*;
use crate::prelude::*;

pub(crate) fn to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

pub struct BindingConfig {
    pub chmgr_api: &'static str,
    pub station: &'static str,
}

pub struct ApiUserData {
    pub uid: &'static str,
    pub station: &'static str,
}

impl AfbApiControls for ApiUserData {
    // the API is created and ready. At this level user may subcall api(s) declare as dependencies
    fn start(&mut self, api: &AfbApi) -> Result<(), AfbError> {

        AfbSubCall::call_sync(api, "OCPP-C", "BootNotification", v106::BootNotification::Request(v106::BootNotificationRequest {
                charge_point_vendor: self.station.to_string(),
                charge_point_model: "Tux-Evse rust OCPP-1.6".to_string(),
                firmware_version: Some("v1234".to_string()),
                charge_box_serial_number: None,
                charge_point_serial_number: None,
                iccid: None,
                imsi: None,
                meter_serial_number: None,
                meter_type: None,
            }))?;
        Ok(())
    }

    // mandatory for downcasting back to custom apidata object
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}


// Binding init callback started at binding load time before any API exist
// -----------------------------------------
pub fn binding_init(rootv4: AfbApiV4, jconf: JsoncObj) -> Result<&'static AfbApi, AfbError> {
    afb_log_msg!(Info, rootv4, "config:{}", jconf);

    let uid = if let Ok(value) = jconf.get::<String>("uid") {
        to_static_str(value)
    } else {
        "ocpp"
    };

    let api = if let Ok(value) = jconf.get::<String>("api") {
        to_static_str(value)
    } else {
        uid
    };

    let info = if let Ok(value) = jconf.get::<String>("info") {
        to_static_str(value)
    } else {
        ""
    };

    let acls = if let Ok(value) = jconf.get::<String>("acls") {
        AfbPermission::new(to_static_str(value))
    } else {
        AfbPermission::new("acl:ocpp:client")
    };


    let station= to_static_str(jconf.get::<String>("station")?);
    let chmgr_api= to_static_str(jconf.get::<String>("chmgr_api")?);

    let config= BindingConfig {
        station,
        chmgr_api,

    };

    // register data converter
    v106::register_datatype()?;
    chmgr_registers()?;

    // create backend API
    let backend = AfbApi::new("OCPP").set_info(info).set_permission(acls);
    register_backend(backend, &config)?;

    // create an register charger api
    let charger = AfbApi::new(api).set_info(info).set_permission(acls);
    register_charger(charger, &config)?;

    backend.finalize()?;
    Ok(charger.finalize()?)
}

// register binding within libafb
AfbBindingRegister!(binding_init);
