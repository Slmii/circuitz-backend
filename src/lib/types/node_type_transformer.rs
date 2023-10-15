use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Transformer {
	pub input: String,
	pub output: String,
}
