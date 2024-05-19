use candid::CandidType;
use serde::{ Deserialize, Serialize };
use crate::impl_storable_for;

impl_storable_for!(ConnectorKey);
#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectorKey {
	pub id: u32,
	pub owner: String,
}
