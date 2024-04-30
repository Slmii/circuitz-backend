use candid::{ CandidType, Principal };
use serde::Deserialize;
use crate::impl_storable_for;

impl_storable_for!(Trace);
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
