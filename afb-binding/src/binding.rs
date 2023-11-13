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
use libocpp::prelude::*;
use crate::prelude::*;

pub(crate) fn to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

pub struct ChargerConfig {
    pub uid: &'static str,
    pub station: &'static str,
}

impl AfbApiControls for ChargerConfig {
    fn config(&mut self, api: &AfbApi, jconf: JsoncObj) -> Result<(), AfbError> {
        afb_log_msg!(Debug, api, "api={} config={}", api.get_uid(), jconf);

        Ok(())
    }

    // mandatory for downcasting back to custom api data object
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


    let station= if let Ok(value) = jconf.get::<String>("station") {
       to_static_str(value)
    } else {
        return Err(AfbError::new ("binding-session-config", "station should be defined in config"))
    };

    let charger_config= ChargerConfig {
        uid,
        station,
    };

    // register data converter
    v106::register_datatype() ?;

    // create backend API
    let backend = AfbApi::new("OCPP").set_info(info).set_permission(acls);
    register_backend(backend, )?;
    backend.finalize()?;

    // create an register charger api
    let charger = AfbApi::new(api).set_info(info).set_permission(acls);
    register_charger(charger, charger_config)?;
    Ok(charger.finalize()?)

}

// register binding within libafb
AfbBindingRegister!(binding_init);
