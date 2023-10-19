use std::borrow::Cow;
use candid::{ CandidType, types::principal::Principal, Decode, Encode };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::Deserialize;

use super::{
	node_type_canister::Canister,
	node_type_lookup::LookupCanister,
	node_pin::Pin,
	node_type_http_request::HttpRequest,
	node_type_transformer::Transformer,
	node_type_mapper::Mapper,
};

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
	/// Canister or HttpRequest will both act as the Input Node
	Canister(Canister),
	HttpRequest(HttpRequest),

	/// Define a transformation rule to and fields to the response data returned by the previous Node, while keeping all other fields
	Transformer(Transformer),
	/// Define one or more mappings to transform the data returned by the Node to different specified fields.
	Mapper(Mapper),
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
