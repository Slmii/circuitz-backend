use candid::CandidType;
use serde::Deserialize;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct NotifyCycles {
	pub is_enabled: bool,
	pub email: String,
	pub threshold: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct NotifyError {
	pub is_enabled: bool,
	pub email: String,
}
