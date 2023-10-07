use candid::Deserialize;
use ic_cdk::api::call::RejectionCode;
use ic_kit::candid::CandidType;

#[derive(CandidType, Clone, Deserialize)]
pub enum ApiError {
	Unauthorized(String),
	NotFound(String),
	AlreadyExists(String),
	CanisterFailed(CanisterFailedError),
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CanisterFailedError {
	pub code: RejectionCode,
	pub message: String,
}
