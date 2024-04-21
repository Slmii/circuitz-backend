use candid::CandidType;
use serde::{ Deserialize, Serialize };
use crate::impl_storable_for;

impl_storable_for!(CircuitKey);
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CircuitKey {
	pub id: u32,
	pub owner: String,
}
