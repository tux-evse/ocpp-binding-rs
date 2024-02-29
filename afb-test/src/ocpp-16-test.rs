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

        let heartbeat_start = AfbTapTest::new("heartbeat-start", self.target, "heartbeat")
            .set_info("send heartbeat to backend")
            .add_arg(123456789)? // provide a nonce
            .finalize()?;

        // start transaction with tagid
        let start_transaction = AfbTapTest::new("transaction-start", self.target, "transaction")
            .set_info("send start transaction")
            .add_arg(OcppTransaction::Start("tux-evse-001".to_string()))?
            .finalize()?;

        let start_charge =
            AfbTapTest::new("notify-charge-start", self.target, "status-notification")
                .set_info("send charging notification")
                .add_arg(OcppChargerStatus::Charging)?
                .finalize()?;

        let mut energy_state = EnergyState {
            subscription_max: 0,
            imax: 0,
            pmax: 0,
            umax: 0,
            session: 5000,
            current: 10000,
            tension: 240000,
            power: 100,
            timestamp: Duration::new(0, 0),
        };

        let send_measure_1 = AfbTapTest::new("engy-mock-state-1", self.target, "engy-state")
            .set_info("send mock measure to backend")
            .set_delay(30000) // wait 5s before pushing this test
            .add_arg(energy_state.clone())? // provide a nonce
            .finalize()?;

        energy_state.session=  energy_state.session+1000;
        let send_measure_2 = AfbTapTest::new("engy-mock-state-2", self.target, "engy-state")
            .set_info("send mock measure to backend")
            .set_delay(30000) // wait 5s before pushing this test
            .add_arg(energy_state.clone())? // provide a nonce
            .finalize()?;

        let finishing_charge = AfbTapTest::new(
            "notify-charge-finishing",
            self.target,
            "status-notification",
        )
        .set_info("send available notification")
        .set_delay(10000) // wait 30s before pushing this test
        .add_arg(OcppChargerStatus::Finishing)?
        .finalize()?;

        // stop transaction send consumes power
        let stop_transaction = AfbTapTest::new("transaction-stop", self.target, "transaction")
            .set_info("send stop transaction")
            .set_delay(10000) // wait 10s before pushing this test
            .add_arg(OcppTransaction::Stop(energy_state.session))?
            .finalize()?;

        let stopped_charge =
            AfbTapTest::new("set_status-available", self.target, "status-notification")
                .set_info("send available notification")
                .add_arg(OcppChargerStatus::Available)?
                .finalize()?;

        // wait before closing the connection (time to check backend->charger request)
        let heartbeat_stop = AfbTapTest::new("heartbeat-stop", self.target, "heartbeat")
            .set_info("send heartbeat to backend")
            .add_arg(987654321)? // provide a nonce
            .set_delay(30000) // wait 30s before pushing this test
            .finalize()?;

        AfbTapSuite::new(api, "Tap Demo Test")
            .set_info("OCPP frontend -> occp server test")
            .set_timeout(0)
            .add_test(heartbeat_start)
            .add_test(tagid_check)
            .add_test(start_transaction)
            .add_test(start_charge)
            .add_test(send_measure_1)
            .add_test(send_measure_2)
            .add_test(finishing_charge)
            .add_test(stop_transaction)
            .add_test(stopped_charge)
            .add_test(heartbeat_stop)
            .set_autorun(self.autostart)
            .set_autoexit(self.autoexit)
            .set_output(self.output)
            .finalize()?;
        Ok(())
    }

    // mandatory for downcasting back to custom apidata object
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

AfbVerbRegister!(ReserveChargerVerb, reserve_charger_cb);
fn reserve_charger_cb(rqt: &AfbRequest, args: &AfbData) -> Result<(), AfbError> {
    let _reservation = args.get::<&ReservationSession>(0)?;
    afb_log_msg!(Notice, rqt, "Mock reservation verb");
    rqt.reply(ReservationStatus::Accepted, 0);
    Ok(())
}

AfbVerbRegister!(ResetChargerVerb, reset_charger_cb);
fn reset_charger_cb(rqt: &AfbRequest, _args: &AfbData) -> Result<(), AfbError> {
    afb_log_msg!(Notice, rqt, "Mock reset verb");
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct SubscribeStateCtx {
    evt: &'static AfbEvent,
}
AfbVerbRegister!(SubscribeStateVerb, subscribe_state_cb, SubscribeStateCtx);
fn subscribe_state_cb(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut SubscribeStateCtx,
) -> Result<(), AfbError> {
    let args = args.get::<&EnergyAction>(0)?;
    afb_log_msg!(Notice, rqt, "Mock engy subscribe state args:{:?}", args);

    match args {
        EnergyAction::SUBSCRIBE => {
            ctx.evt.subscribe(rqt)?;
        }

        EnergyAction::UNSUBSCRIBE => {
            ctx.evt.unsubscribe(rqt)?;
        }
        _ => return afb_error!("mock-engy-state", "invalid request:{:?}", args),
    }

    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct MockEngyEvtCtx {
    evt: &'static AfbEvent,
    total: i32,
}
// this verb is only for testing purpose real measure should be send from engy event
AfbVerbRegister!(EngyMockRqt, engy_state_request, MockEngyEvtCtx);
fn engy_state_request(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut MockEngyEvtCtx,
) -> Result<(), AfbError> {
    let power = args.get::<i32>(0)?;
    ctx.total = ctx.total + power;

    afb_log_msg!(Debug, rqt, "EngyStateVerb request power:{}", power);
    let engy_state = EnergyState {
        subscription_max: 0,
        imax: 0,
        pmax: 0,
        umax: 0,
        session: ctx.total,
        current: power / 240,
        tension: 240,
        power: power,
        timestamp: Duration::new(0, 0),
    };
    ctx.evt.push(engy_state);
    rqt.reply(AFB_NO_DATA, 0);
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

    let state_event = AfbEvent::new("push-mesure");
    let push_verb = AfbVerb::new("energy-state")
        .set_name("push-mesure")
        .set_info("Mock current energy state event")
        .set_action("['subscribe','unsubscribe']")?
        .set_callback(Box::new(MockEngyEvtCtx {
            evt: state_event,
            total: 0,
        }))
        .finalize()?;

    let state_verb = AfbVerb::new("subscribe-state")
        .set_name("state")
        .set_info("Mock engy state subscription")
        .set_callback(Box::new(SubscribeStateVerb { evt: state_event }))
        .finalize()?;

    let reserve_verb = AfbVerb::new("reserve-charger")
        .set_name("reserve")
        .set_info("Mock reserve charging manager api")
        .set_callback(Box::new(ReserveChargerVerb {}))
        .finalize()?;

    let reset_verb = AfbVerb::new("reset-charger")
        .set_name("reset")
        .set_info("Mock reset charging manager api")
        .set_callback(Box::new(ResetChargerVerb {}))
        .finalize()?;

    afb_log_msg!(Notice, rootv4, "ocpp test uid:{} target:{}", uid, target);
    let api = AfbApi::new(uid)
        .set_info("Testing OCPP tap reporting")
        //.require_api(target) introduce a loop in dependency
        .set_callback(Box::new(tap_config))
        .add_verb(push_verb)
        .add_verb(state_verb)
        .add_verb(reserve_verb)
        .add_verb(reset_verb)
        .add_event(state_event)
        .seal(false)
        .finalize()?;
    Ok(api)
}

// register rootv4 within libafb
AfbBindingRegister!(binding_test_init);
