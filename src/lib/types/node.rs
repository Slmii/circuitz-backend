use std::{ borrow::Cow, collections::HashMap };
use candid::{ CandidType, types::principal::Principal, Decode, Encode };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::{ Deserialize, Serialize };
use super::headers::Headers;

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
	pub pin: Vec<Pin>,
	pub created_at: u64,
	pub updated_at: u64,
}

impl Storable for Node {
	fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
		Cow::Owned(Encode!(self).unwrap())
	}

	fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
		Decode!(bytes.as_ref(), Self).unwrap()
	}

	const BOUND: Bound = Bound::Unbounded;
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
			pin: Default::default(),
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
pub struct Mapper {
	input: String,
	output: String,
	// Either upload an IDL and read the fields or make a 'sample' request and read the fields
	interface: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupCanister {
	pub name: String,
	pub description: Option<String>,
	pub canister: Principal,
	pub method: String,
	pub args: Vec<Arg>,
	pub cycles: u128,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupHttpRequest {
	pub name: String,
	pub description: Option<String>,
	pub url: String,
	pub headers: Headers,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arg {
	String(String),
	Number(u32),
	Principal(Principal),
	BigInt(u64),
	Boolean(bool),
	Array(Vec<Arg>),
	Object(HashMap<String, Arg>),
	Field(String),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HttpRequest {
	name: String,
	description: Option<String>,
	url: String,
	method: HttpRequestMethod,
	// Store header name and value
	headers: Headers,
	request_body: Option<String>,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum HttpRequestMethod {
	GET,
	POST,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Canister {
	name: String,
	verification_type: VerificationType,
	description: Option<String>,
	sample_data: Option<String>,
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
	FilterPin(FilterPin),
	/// You can use this Pin to transform the data within a Node after a Node request.
	LookupTransformPin(LookupTransformPin),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CustomPinLogic {
	function: Option<String>,
	script: Option<String>,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FilterPin {
	rules: Vec<Rule>,
	condition: Condition,
	condition_group: Option<ConditionGroup>,
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
