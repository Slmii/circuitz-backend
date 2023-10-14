use candid::{ Principal, CandidType };
use serde::Deserialize;

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
