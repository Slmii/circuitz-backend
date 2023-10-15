use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Mapper {
	input: String,
	output: String,
	// Either upload an IDL and read the fields or make a 'sample' request and read the fields
	interface: String,
}
