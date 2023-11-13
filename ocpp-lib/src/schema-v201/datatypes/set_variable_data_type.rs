use super::component_type::ComponentType;
use super::variable_type::VariableType;
use crate::v201::enumerations::attribute_enum_type::AttributeEnumType;

/// SetVariableDataType is used by: SetVariablesRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    pub attribute_value: String,
    pub component: ComponentType,
    pub variable: VariableType,
}
