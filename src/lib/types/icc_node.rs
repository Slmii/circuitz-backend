use candid::CandidType;
use serde::{ Deserialize, Serialize };

use super::node::Arg;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ICCPost {
	pub canisterId: String,
	pub methodName: String,
	pub args: Vec<Arg>,
}

// This struct is legacy code and is not really used in the code.
#[derive(Serialize, Deserialize)]
pub struct Context {
	pub bucket_start_time_index: usize,
	pub closing_price_index: usize,
}
