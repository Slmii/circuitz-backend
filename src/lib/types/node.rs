use std::{ borrow::Cow, collections::HashMap };
use candid::{ CandidType, Principal, Decode, Encode };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::Deserialize;

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
			node_type: NodeType::Transformer(Transformer {
				input: Default::default(),
				output: Default::default(),
			}),
			pin: Default::default(),
			created_at: Default::default(),
			updated_at: Default::default(),
		}
	}
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum NodeType {
	Canister(Canister),
	/// Define a transformation rule to and fields to the response data returned by the previous Node, while keeping all other fields
	Transformer(Transformer),
	/// Define one or more mappings to transform the data returned by the Node to different specified fields.
	Mapper(Mapper),
	Ouput(Ouput),
	/// Define a request to retrieve/post data from/to a different endpoint.
	Request(Request),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Ouput {
	name: String,
	description: Option<String>,
	canister: Principal,
	method: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Canister {
	name: String,
	verification_type: VerificationType,
	description: Option<String>,
	sample_data: Option<String>,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Request {
	name: String,
	description: Option<String>,
	// Request can be either fetching data or posting data
	request_type: RequestType,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum RequestType {
	RequestCanister(RequestCanister),
	RequestHttp(RequestHttp),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RequestCanister {
	canister: Principal,
	method: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RequestHttp {
	url: String,
	method: RequestHttpMethod,
	// Store header name and value
	headers: HashMap<String, String>,
	request_body: Option<String>,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum RequestHttpMethod {
	GET,
	POST,
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
	PostResponsePin(CustomPinLogic),
	/// You can use this Pin map data within a Node to a different format. A MapperPin will always be the first Pin to be executed within a Node.
	MapperPin(Mapper),
	/// You can use this Pin to filter the Node from being executed. A FilterPin will always be executed before the Node is executed.
	FilterPin(Vec<ConditionGroup>),
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
pub struct Transformer {
	input: String,
	output: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Mapper {
	input: String,
	output: String,
	// Either upload an IDL and read the fields or make a 'sample' request and read the fields
	interface: String,
}
