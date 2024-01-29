/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk samples code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
 * Debug: wireshark -i eth0 -k -S -f "host ocpp.biastaging.com and tcp port 80"
 */

use afbv4::prelude::*;
use ocpp::prelude::*;
use std::time::Duration;
use typesv4::prelude::*;

// This rootv4 demonstrate how to test an external rootv4 that you load within the same afb-binder process and security context
// It leverages test (Test Anything Protocol) that is compatible with redpesk testing report.
struct TapUserData {
    autostart: bool,
    autoexit: bool,
    output: AfbTapOutput,
    target: &'static str,
}

// AfbApi userdata implements AfbApiControls trait
impl AfbApiControls for TapUserData {
    fn start(&mut self, api: &AfbApi) -> Result<(), AfbError> {
        afb_log_msg!(Notice, api, "starting OCPP-16 testing");

        // check tad_id on server
        let tagid_check = AfbTapTest::new("authentication-request", self.target, "authorize")
            .set_info("send authentication request")
            .add_arg("tux-evse-001")?
            .finalize()?;

        // start transaction with tagid
        let start_transaction = AfbTapTest::new("transaction-start", self.target, "transaction")
            .set_info("send start transaction")
            .add_arg(OcppTransaction::Start("tux-evse-001".to_string()))?
            .finalize()?;

        let send_measure = AfbTapTest::new("engy-mock-state", self.target, "engy-state")
            .set_info("send mock measure to backend")
            .add_arg(EnergyState {
                subscription_max: 0,
                imax: 0,
                pmax: 0,
                tension_max: 0,
                session: 102400,
                total: 409600,
                current: 1500,
                tension: 24000,
                power: 2200,
                timestamp: Duration::new(0, 0),
            })? // provide a nonce
            .finalize()?;

        // stop transaction send consumes power
        let stop_transaction = AfbTapTest::new("transaction-stop", self.target, "transaction")
            .set_info("send stop transaction")
            .add_arg(OcppTransaction::Stop(1234))?
            .finalize()?;

        let heartbeat = AfbTapTest::new("heartbeat", self.target, "heartbeat")
            .set_info("send heartbeat to backend")
            .add_arg(123456789)? // provide a nonce
            .finalize()?;

        AfbTapSuite::new(api, "Tap Demo Test")
            .set_info("OCPP frontend -> occp server test")
            .set_timeout(0)
            .add_test(heartbeat)
            .add_test(tagid_check)
            .add_test(start_transaction)
            .add_test(send_measure)
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

// lock engy+chmgr APIs
struct StateRequestCtx {
    evt: &'static AfbEvent,
}
AfbVerbRegister!(StateRequestVerb, state_request_cb, StateRequestCtx);
fn state_request_cb(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut StateRequestCtx,
) -> Result<(), AfbError> {
    match args.get::<&EnergyAction>(0)? {
        EnergyAction::SUBSCRIBE => {
            afb_log_msg!(Notice, rqt, "Subscribe {}", ctx.evt.get_uid());
            ctx.evt.subscribe(rqt)?;
            rqt.reply(AFB_NO_DATA, 0);
        }

        EnergyAction::UNSUBSCRIBE => {
            afb_log_msg!(Notice, rqt, "Unsubscribe {}", ctx.evt.get_uid());
            ctx.evt.unsubscribe(rqt)?;
            rqt.reply(AFB_NO_DATA, 0);
        }

        _ => {
            return afb_error!(
                "energy-state-action",
                "unsupported action should be (subscribe|unsubscribe)"
            )
        }
    }
    Ok(())
}

AfbVerbRegister!(ReservechargerVerb, reserve_charger_cb);
fn reserve_charger_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let _reservation = args.get::<&ReservationSession>(0)?;
    rqt.reply(ReservationStatus::Accepted, 0);
    Ok(())
}

// rootv4 init callback started at rootv4 load time before any API exist
// -----------------------------------------
pub fn binding_test_init(rootv4: AfbApiV4, jconf: JsoncObj) -> Result<&'static AfbApi, AfbError> {
    let uid = jconf.get::<&'static str>("uid")?;
    let target = jconf.get::<&'static str>("target")?;

    let tap_config = TapUserData {
        autostart: jconf.default::<bool>("autostart", true)?,
        autoexit: jconf.default::<bool>("autoexit", true)?,
        output: AfbTapOutput::TAP,
        target,
    };

    // custom type should register once per binder
    v106::register_datatype()?;
    ocpp_registers()?;
    engy_registers()?;

    let state_event = AfbEvent::new("engy-state");
    let state_verb = AfbVerb::new("charging-state")
        .set_name("state")
        .set_info("current charging state")
        .set_action("['read','subscribe','unsubscribe']")?
        .set_callback(Box::new(StateRequestCtx { evt: state_event }))
        .finalize()?;

    let reserve_verb = AfbVerb::new("reserve-charger")
        .set_name("reserve")
        .set_info("reserve charging station start/stop data")
        .set_action("['now','delay','cancel']")?
        .set_callback(Box::new(ReservechargerVerb {}))
        .finalize()?;

    afb_log_msg!(Notice, rootv4, "ocpp test uid:{} target:{}", uid, target);
    let api = AfbApi::new(uid)
        .set_info("Testing OCPP tap reporting")
        .require_api(target)
        .set_callback(Box::new(tap_config))
        .add_event(state_event)
        .add_verb(state_verb)
        .add_verb(reserve_verb)
        .seal(false)
        .finalize()?;
    Ok(api)
}

// register rootv4 within libafb
AfbBindingRegister!(binding_test_init);
