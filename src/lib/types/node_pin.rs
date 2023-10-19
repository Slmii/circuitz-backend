use candid::CandidType;
use serde::Deserialize;

use super::node_type_mapper::Mapper;

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Pin {
	pin_type: PinType,
	order: u32,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum PinType {
	/// You can use this Pin to apply JS logic to the data within a Node prior starting the Node.
	PrePin(CustomPinLogic),
	/// You can use this Pin to apply JS logic to the data within a Node after the Node has finished.
	PostPin(CustomPinLogic),
	/// You can use this Pin map data within a Node to a different format
	MapperPin(Mapper),
	/// You can use this Pin to filter the Node from being executed
	FilterPin(Vec<ConditionGroup>),
	/// You can use this Pin to transform the data within a Node after a Node request.
	LookupTransformPin(LookupTransformPin),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CustomPinLogic {
	function: Option<String>,
	script: Option<String>,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ConditionGroup {
	condition: Condition,
	condition_group_type: Option<ConditionGroupType>,
	field: String,
	operator: Operator,
	value: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum Condition {
	Not,
	Is,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum ConditionGroupType {
	And,
	Or,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum Operator {
	Equal,
	NotEqual,
	GreaterThan,
	LessThan,
	GreaterThanOrEqual,
	LessThanOrEqual,
	In,
	NotIn,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupTransformPin {
	input: String,
	output: String,
}
