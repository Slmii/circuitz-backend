use std::collections::HashMap;

use candid::{ Principal, CandidType };
use serde::Deserialize;

use super::headers::Headers;

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupCanister {
	pub name: String,
	pub description: Option<String>,
	pub canister: Principal,
	pub method: String,
	pub args: Vec<Arg>,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupHttpRequest {
	pub name: String,
	pub description: Option<String>,
	pub url: String,
	pub headers: Headers,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum Arg {
	String(String),
	Number(u32),
	BigInt(u64),
	Boolean(bool),
	Array(Vec<Arg>),
	Object(HashMap<String, Arg>),
}
