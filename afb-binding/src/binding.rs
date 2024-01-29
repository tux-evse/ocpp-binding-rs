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

pub struct BindingConfig {
    pub chmgr_api: &'static str,
    pub engy_api: &'static str,
    pub station: &'static str,
    pub mgr: &'static ManagerHandle,
    pub cid: u32,
}

pub struct ApiUserData {
    pub mgr: &'static ManagerHandle,
    pub station: &'static str,
    pub evt: &'static AfbEvent,
    pub tic: u32,
}

impl AfbApiControls for ApiUserData {
    // the API is created and ready. At this level user may subcall api(s) declare as dependencies
    fn start(&mut self, api: &AfbApi) -> Result<(), AfbError> {
        ocpp_bootstrap(api, self.station, self.tic)?;
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

    let uid =  jconf.default::<&'static str>("uid", "ocpp-16")?;
    let api =  jconf.default::<&'static str>("api", uid)?;
    let info =  jconf.default::<&'static str>("info", "")?;
    let cid =  jconf.default::<u32>("cid",1)?;
    let tic = jconf.default::<u32>("tic",0)?;
    let station = jconf.default::<&'static str>("station","tux-evse")?;
    let chmgr_api = jconf.default::<&'static str>("chmgr_api", "")?;
    let engy_api = jconf.default::<&'static str>("engy_api", "")?;

    // register data converter
    v106::register_datatype()?;
    chmgr_registers()?;
    ocpp_registers()?;
    engy_registers()?;

    // create occp manager
    let evt = AfbEvent::new("msg");
    let mgr = ManagerHandle::new(rootv4, cid, evt, chmgr_api);
    let config = BindingConfig {
        station,
        chmgr_api,
        engy_api,
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
        .set_callback(Box::new(ApiUserData { mgr, station, evt, tic }));

    register_frontend(frontend, &config)?;

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
