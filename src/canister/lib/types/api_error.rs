use candid::Deserialize;
use candid::CandidType;

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum ApiError {
	Unauthorized(String),
	NotFound(String),
	AlreadyExists(String),
	InterCanister(String),
}
