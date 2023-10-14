use candid::{ Principal, CandidType };
use serde::Deserialize;

use super::headers::Headers;

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Lookup {
	name: String,
	description: Option<String>,
	lookup_type: LookupType,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum LookupType {
	LookupCanister(LookupCanister),
	LookupHttp(LookupHttp),
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupCanister {
	canister: Principal,
	method: String,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LookupHttp {
	url: String,
	// Store header name and value
	headers: Headers,
}
