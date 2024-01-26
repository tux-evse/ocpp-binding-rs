/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk samples code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 */


use afbv4::prelude::*;
use ocpp::prelude::*;
use typesv4::prelude::*;

// This rootv4 demonstrate how to test an external rootv4 that you load within the same afb-binder process and security context
// It leverages test (Test Anything Protocol) that is compatible with redpesk testing report.
struct TapUserData {
    autostart: bool,
    autoexit: bool,
    output: AfbTapOutput,
}

// AfbApi userdata implements AfbApiControls trait
impl AfbApiControls for TapUserData {
    fn start(&mut self, api: &AfbApi) -> Result<(), AfbError> {
        afb_log_msg!(Notice, api, "starting OCPP testing");

        // check tad_id on server
        let tagid_check = AfbTapTest::new("authentication-request", "ocpp", "authorize")
            .set_info("send authentication request")
            .add_arg("tux-evse-001")?
            .finalize()?;

        // start transaction with tagid
        let start_transaction = AfbTapTest::new("transaction-start", "ocpp", "transaction")
            .set_info("send start transaction")
            .add_arg(OcppTransaction::Start("tux-evse-001".to_string()))?
            .finalize()?;

        // stop transaction send consumes power
        let stop_transaction = AfbTapTest::new("transaction-stop", "ocpp", "transaction")
            .set_info("send stop transaction")
            .add_arg(OcppTransaction::Stop(1234))?
            .finalize()?;

        let heartbeat = AfbTapTest::new("heartbeat", "ocpp", "heartBeat")
            .set_info("send heartbeat to backend")
            .finalize()?;

        AfbTapSuite::new(api, "Tap Demo Test")
            .set_info("OCPP frontend -> occp server test")
            .set_timeout(0)
            .add_test(heartbeat)
            .add_test(tagid_check)
            .add_test(start_transaction)
            .add_test(stop_transaction)
            .set_autorun(self.autostart)
            .set_autoexit(self.autoexit)
            .set_output(self.output)
            .finalize()?;
        Ok(())
    }

    fn config(&mut self, api: &AfbApi, jconf: JsoncObj) -> Result<(), AfbError> {
        afb_log_msg!(Debug, api, "api={} config={}", api.get_uid(), jconf);
        match jconf.get::<bool>("autostart") {
            Ok(value) => self.autostart = value,
            Err(_error) => {}
        };

        match jconf.get::<bool>("autoexit") {
            Ok(value) => self.autoexit = value,
            Err(_error) => {}
        };

        match jconf.get::<String>("output") {
            Err(_error) => {}
            Ok(value) => match value.to_uppercase().as_str() {
                "JSON" => self.output = AfbTapOutput::JSON,
                "TAP" => self.output = AfbTapOutput::TAP,
                "NONE" => self.output = AfbTapOutput::NONE,
                _ => {
                    afb_log_msg!(
                        Error,
                        api,
                        "Invalid output should be json|tap (default used)"
                    );
                }
            },
        };

        Ok(())
    }

    // mandatory for downcasting back to custom apidata object
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

// rootv4 init callback started at rootv4 load time before any API exist
// -----------------------------------------
pub fn binding_test_init(rootv4: AfbApiV4, jconf: JsoncObj) -> Result<&'static AfbApi, AfbError> {
    let uid = match jconf.get::<String>("uid") {
        Ok(value) => value,
        Err(_error) => "Tap-test-rootv4".to_owned(),
    };

    let tap_config = TapUserData {
        autostart: true,
        autoexit: true,
        output: AfbTapOutput::TAP,
    };

    // custom type should register once per binder
    v106::register_datatype()?;

    afb_log_msg!(Notice, rootv4, "-- rootv4 {} loaded", uid);
    let api = AfbApi::new("ocpp-test")
        .set_info("Testing OCPP tap reporting")
        .require_api("ocpp-106")
        .set_callback(Box::new(tap_config))
        .seal(false)
        .finalize()?;
    Ok(api)
}

// register rootv4 within libafb
AfbBindingRegister!(binding_test_init);
