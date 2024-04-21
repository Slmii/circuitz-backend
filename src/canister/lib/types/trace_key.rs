use candid::CandidType;
use serde::{ Deserialize, Serialize };
use crate::impl_storable_for;

impl_storable_for!(TraceKey);
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraceKey {
	pub id: u32,
	pub circuit_id: u32,
	pub node_id: u32,
	pub owner: String,
}
