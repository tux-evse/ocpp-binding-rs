use crate::v201::enumerations::cost_kind_enum_type::CostKindEnumType;

/// CostType is used by: Common:ConsumptionCostType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CostType {
    pub cost_kind: CostKindEnumType,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_multiplier: Option<i8>,
}
