//! ChargingNeedsType
use chrono::{DateTime, Utc};

use super::ac_charging_parameters_type::ACChargingParametersType;
use super::dc_charging_parameters_type::DCChargingParametersType;
use crate::v201::enumerations::energy_transfer_mode_enum_type::EnergyTransferModeEnumType;

/// ChargingNeedsType
///
/// ChargingNeedsType is used by: [NotifyEVChargingNeedsRequest](`crate::v201::messages::notify_ev_charging_needs::NotifyEVChargingNeedsRequest`)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    /// Mode of energy transfer requested by the EV.
    pub requested_energy_transfer: EnergyTransferModeEnumType,
    /// Estimated departure time of the EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,
    /// EV AC charging parameters.
    #[serde(rename = "acChargingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,
    /// EV DC charging parameters
    #[serde(rename = "dcChargingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,
}
