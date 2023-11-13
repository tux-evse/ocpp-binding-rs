use super::component_type::ComponentType;
use super::status_info_type::StatusInfoType;
use super::variable_type::VariableType;
use crate::v201::enumerations::attribute_enum_type::AttributeEnumType;
use crate::v201::enumerations::set_variable_status_enum_type::SetVariableStatusEnumType;

/// SetVariableResultType is used by: SetVariablesResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    pub attribute_status: SetVariableStatusEnumType,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}
