use std::collections::HashMap;
use candid::{ CandidType, types::principal::Principal };
use serde::{ Deserialize, Serialize };
use crate::impl_storable_for;
use super::headers::Headers;

impl_storable_for!(Node);
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Node {
	pub id: u32,
	pub user_id: Principal,
	pub circuit_id: u32,
	pub order: u32,
	pub is_enabled: bool,
	pub is_error: bool,
	pub is_running: bool,
	pub node_type: NodeType,
	// AKA "hooks"
	pub pins: Vec<Pin>,
	pub created_at: u64,
	pub updated_at: u64,
}

impl Default for Node {
	fn default() -> Self {
		Self {
			id: Default::default(),
			user_id: Principal::anonymous(),
			circuit_id: Default::default(),
			order: Default::default(),
			is_enabled: Default::default(),
			is_error: Default::default(),
			is_running: Default::default(),
			node_type: NodeType::Canister(Canister {
				description: Default::default(),
				name: Default::default(),
				sample_data: Default::default(),
				verification_type: VerificationType::None,
			}),
			pins: Default::default(),
			created_at: Default::default(),
			updated_at: Default::default(),
		}
	}
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum NodeType {
	/// Canister or HttpRequest will both act as the Input Node
	Canister(Canister),
	HttpRequest(HttpRequest),

	Output(Output),

	/// Define a lookup request to retrieve data from a different endpoint.
	LookupCanister(LookupCanister),
	LookupHttpRequest(HttpRequest),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Output {
	name: String,
	description: Option<String>,
	canister: Principal,
	method: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Transformer {
	pub input: String,
	pub output: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupCanister {
	pub name: String,
	pub description: Option<String>,
	pub canister: Principal,
	pub method: String,
	pub args: Vec<Arg>,
	pub cycles: u128,
	pub sample_data: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupHttpRequest {
	pub url: String,
	pub method: HttpRequestMethod,
	pub headers: Headers,
	pub request_body: Option<String>,
	pub cycles: u128,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arg {
	String(String),
	Number(String),
	Principal(String),
	BigInt(String),
	Boolean(String),
	Array(String),
	Object(String),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HttpRequest {
	name: String,
	description: Option<String>,
	pub url: String,
	pub method: HttpRequestMethod,
	pub headers: Headers,
	pub request_body: Option<String>,
	pub cycles: u128,
	sample_data: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpRequestMethod {
	GET,
	POST,
	PUT,
	DELETE,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Canister {
	name: String,
	verification_type: VerificationType,
	description: Option<String>,
	sample_data: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum VerificationType {
	None,
	Token(Token),
	Whitelist(Vec<Principal>),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Token {
	token: String,
	field: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Pin {
	pub pin_type: PinType,
	pub order: u32,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum PinType {
	/// You can use this Pin to apply JS logic to the data within a Node prior starting the Node.
	PrePin(CustomPinLogic),
	/// You can use this Pin to apply JS logic to the data within a Node after the Node has finished.
	PostPin(CustomPinLogic),
	/// You can use this Pin map data within a Node to a different format
	PreMapperPin(MapperPin),
	PostMapperPin(MapperPin),
	/// You can use this Pin to filter the Node from being executed
	FilterPin(FilterPin),
	/// You can use this Pin to transform the data within a Node after a Node request.
	LookupTransformPin(LookupTransformPin),
	/// You can use this Pin to specify which data returned by the LookUp get merged into the Node.
	LookupFilterPin(FilterPin),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CustomPinLogic {
	function: Option<String>,
	script: Option<String>,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MapperPin {
	// Input and Output
	fields: Vec<(String, String)>,
	sample_data: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FilterPin {
	rules: Vec<Rule>,
	condition: Condition,
	condition_group: Option<ConditionGroup>,
	sample_data: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Rule {
	field: String,
	operator: Operator,
	value: String,
	operand: Operand,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum Condition {
	Not,
	Is,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum ConditionGroup {
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
	Contains,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Operand {
	operand_type: OperandType,
	data_type: DataType,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum OperandType {
	Value,
	Field,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum DataType {
	String,
	Number,
	Principal,
	BigInt,
	Boolean,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupTransformPin {
	input: String,
	output: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupCanisterPreview {
	pub canister: Principal,
	pub method: String,
	pub args: Vec<PreviewArg>,
	pub cycles: u128,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreviewArg {
	String(String),
	Number(u32),
	Principal(Principal),
	BigInt(u64),
	Boolean(bool),
	Array(Vec<Arg>),
	Object(HashMap<String, Arg>),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LookupHttpRequestPreview {
	pub url: String,
	pub method: HttpRequestMethod,
	pub headers: Headers,
	pub request_body: Option<String>,
	pub cycles: u128,
}
