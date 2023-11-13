use crate::v201::enumerations::charging_state_enum_type::ChargingStateEnumType;
use crate::v201::enumerations::reason_enum_type::ReasonEnumType;

/// TransactionType is used by: TransactionEventRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent_charging: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<ReasonEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_start_id: Option<i64>,
}
