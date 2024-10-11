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
        Ok(())
    }

    // mandatory for downcasting back to custom apidata object
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub fn binding_init(rootv4: AfbApiV4, jconf: JsoncObj) -> Result<&'static AfbApi, AfbError> {
    afb_log_msg!(Info, rootv4, "config:{}", jconf);

    let uid = jconf.default::<&'static str>("uid", "ocpp-16")?;
    let api = jconf.default::<&'static str>("api", uid)?;
    let info = jconf.default::<&'static str>("info", "")?;
    let cid = jconf.default::<u32>("cid", 1)?;
    let tic = jconf.default::<u32>("tic", 0)?;
    let station = jconf.default::<&'static str>("station", "tux-evse")?;
    let chmgr_api = jconf.default::<&'static str>("chmgr_api", "")?;
    let engy_api = jconf.default::<&'static str>("engy_api", "")?;

    // Register custom data converters and required APIs
    v106::register_datatype()?;
    chmgr_registers()?;

    // Create event handler
    let event = AfbEvent::new("msg");
    // Create manager handle
    let mgr = ManagerHandle::new(cid, event);

    // Create binding configuration
    let config = BindingConfig {
        station,
        chmgr_api,
        engy_api,
        mgr,
        cid,
    };

    // Create and register frontend
    let frontend = AfbApi::new(api)
        .set_info(info)
        .add_event(event);
    register_frontend(frontend, &config)?;

    Ok(frontend.finalize()?)
}