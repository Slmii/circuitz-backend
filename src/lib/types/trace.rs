use std::borrow::Cow;
use candid::{ CandidType, Principal, Decode, Encode };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraceKey {
	pub id: u32,
	pub circuit_id: u32,
	pub node_id: u32,
	pub owner: String,
}

impl Storable for TraceKey {
	fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
		Cow::Owned(Encode!(self).unwrap())
	}

	fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
		Decode!(bytes.as_ref(), Self).unwrap()
	}

	const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Trace {
	pub id: u32,
	pub user_id: Principal,
	pub node_id: u32,
	pub circuit_id: u32,
	pub status: TraceStatus,
	pub errors: Vec<TraceError>,
	// Stringified JSON
	pub data: String,
	pub duration: u32,
	pub started_at: u64,
	pub completed_at: u64,
	pub created_at: u64,
	pub updated_at: u64,
}

impl Storable for Trace {
	fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
		Cow::Owned(Encode!(self).unwrap())
	}

	fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
		Decode!(bytes.as_ref(), Self).unwrap()
	}

	const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum TraceStatus {
	Success,
	Failed,
	Cancelled,
	InProgress,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TraceError {
	pub code: String,
	pub message: String,
	pub source: String,
	pub resolved_at: Option<u64>,
	pub created_at: u64,
	pub updated_at: u64,
}
