/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk samples code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 */

#![doc(
    html_logo_url = "https://iot.bzh/images/defaults/company/512-479-max-transp.png",
    html_favicon_url = "https://iot.bzh/images/defaults/favicon.ico"
)]

#[cfg(not(afbv4))]
extern crate afbv4;

use afbv4::prelude::*;
use libocpp::prelude::*;
use chrono::Utc;

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

        let boot_notification = AfbTapTest::new("boot-notification", "ocpp-16", "BootNotification")
            .set_info("send boot notification to backend")
            .add_arg(v106::BootNotification::Request(v106::BootNotificationRequest {
                charge_point_vendor: "Tux-EvSe".to_string(),
                charge_point_model: "ocpp-16 Rust-ocpp test".to_string(),
                firmware_version: Some("v1234".to_string()),
                charge_box_serial_number: None,
                charge_point_serial_number: None,
                iccid: None,
                imsi: None,
                meter_serial_number: None,
                meter_type: None,
            }))?
            .add_expect(&JsonStr("{'status':'Accepted'}"))?
            .finalize()?;

        let status_notification =
            AfbTapTest::new("status-notification", "ocpp-16", "StatusNotification")
                .set_info("send status notification to backend")
                .add_arg(v106::StatusNotification::Request(
                    v106::StatusNotificationRequest {
                        connector_id: 1,
                        status: v106::ChargePointStatus::Preparing,
                        error_code: v106::ChargePointErrorCode::NoError,
                        info: Some("Dummy Rust ocpp status report".to_string()),
                        timestamp: Some(Utc::now()),
                        vendor_id: None,
                        vendor_error_code: None,
                    },
                ))?
                // does not returned any status
                .finalize()?;

        let heartbeat = AfbTapTest::new("heartbeat", "ocpp-16", "HeartBeat")
            .set_info("send heartbeat to backend")
            .add_arg(v106::Heartbeat::Request(v106::HeartbeatRequest{
            }))?
            .finalize()?;

        let start_transaction = AfbTapTest::new("start-transaction", "ocpp-16", "StartTransaction")
            .set_info("send start transaction to backend")
            .add_arg(v106::StartTransaction::Request(v106::RemoteStartTransactionRequest{
                id_tag: "Tux-EvSe-Tag".to_string(),
                connector_id:1,
                charging_profile: None,
            }))?
            .add_expect(&JsonStr("{'status':'Accepted'}"))?
            .finalize()?;

        let stop_transaction = AfbTapTest::new("stop-transaction", "ocpp-16", "StopTransaction")
            .set_info("send start transaction to backend")
            .add_arg(v106::StopTransaction::Request(v106::RemoteStopTransactionRequest{
                transaction_id: 1234,
            }))?
            .add_expect(&JsonStr("{'status':'Accepted'}"))?
            .finalize()?;

        AfbTapSuite::new(api, "Tap Demo Test")
            .set_info("Check Example demo API works")
            .set_timeout(0)
            .add_test(boot_notification)
            .add_test(status_notification)
            .add_test(heartbeat)
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
