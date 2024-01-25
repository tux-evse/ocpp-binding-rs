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
use ocpp::prelude::*;
use typesv4::prelude::*;

pub(crate) fn to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

pub struct BindingConfig {
    pub chmgr_api: &'static str,
    pub engy_api: &'static str,
    pub station: &'static str,
    pub tic: u32,
    pub mgr: &'static ManagerHandle,
    pub cid: u32,
}

pub struct ApiUserData {
    pub mgr: &'static ManagerHandle,
    pub station: &'static str,
    pub evt: &'static AfbEvent,
}

impl AfbApiControls for ApiUserData {
    // the API is created and ready. At this level user may subcall api(s) declare as dependencies
    fn start(&mut self, api: &AfbApi) -> Result<(), AfbError> {
        ocpp_bootstrap(api, self.station)?;
        self.evt.push (OcppMsg::Initialized);
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

    let cid = if let Ok(value) = jconf.get::<u32>("cid") {
        value
    } else {
        0
    };

    let tic = jconf.get::<u32>("tic")?;
    let station = to_static_str(jconf.get::<String>("station")?);
    let chmgr_api = to_static_str(jconf.get::<String>("chmgr_api")?);
    let engy_api = to_static_str(jconf.get::<String>("engy_api")?);


    // register data converter
    v106::register_datatype()?;
    chmgr_registers()?;
    ocpp_registers()?;
    engy_registers()?;

    // create occp manager
    let evt = AfbEvent::new("msg");
    let mgr = ManagerHandle::new(cid, evt, chmgr_api);
    let config = BindingConfig {
        station,
        chmgr_api,
        engy_api,
        tic,
        mgr,
        cid,
    };


    // create backend API (OCPP upercase is impose by transport extension)
    let backend = AfbApi::new("OCPP").set_info(info);
    register_backend(backend, &config)?;

    // create an register frontend api and register init session callback
    let frontend = AfbApi::new(api)
        .set_info(info)
        .add_event(evt)
        .set_callback(Box::new(ApiUserData { mgr, station, evt }));

    register_frontend(rootv4, frontend, &config)?;

    // if acls set apply them
    if let Ok(value) = jconf.get::<String>("permission") {
        let perm = to_static_str(value);
        backend.set_permission(AfbPermission::new(perm));
        frontend.set_permission(AfbPermission::new(perm));
    };

    if let Ok(value) = jconf.get::<i32>("verbosity") {
        backend.set_verbosity(value);
        frontend.set_verbosity(value);
    };

    backend.finalize()?;
    Ok(frontend.finalize()?)
}

// register binding within libafb
AfbBindingRegister!(binding_init);
