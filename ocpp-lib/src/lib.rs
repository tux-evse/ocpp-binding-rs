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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * Reference:
 *   https://github.com/codelabsab/ocpp-csms-server
 *   https://github.com/codelabsab/rust-ocpp
 */

#![doc(
    html_logo_url = "https://iot.bzh/images/defaults/company/512-479-max-transp.png",
    html_favicon_url = "https://iot.bzh/images/defaults/favicon.ico"
)]

#[cfg(not(afbv4))]
extern crate afbv4;

#[path = "enums-v106.rs"]
pub mod v106;

#[path = "enums-v201.rs"]
pub mod v201;

// #[path = "schema-v106/mod.rs"]
// pub mod v106;

// #[path = "schema-v201/mod.rs"]
// pub mod v201;

// export to external crate restricted to session APIs
pub mod prelude {
    pub use crate::v106;
    pub use crate::v201;
    //pub use crate::msg::*;
}