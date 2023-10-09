use candid::{ CandidType, Deserialize, Principal };
use serde::Serialize;

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct Circuit {
	pub id: u32,
	pub user_id: Principal,
	pub name: String,
	pub description: Option<String>,
	pub is_favorite: bool,
	pub is_enabled: bool,
	pub run_at: Option<u64>,
	pub created_at: u64,
	pub updated_at: u64,
}

impl Default for Circuit {
	fn default() -> Self {
		Self {
			id: Default::default(),
			user_id: Principal::anonymous(),
			name: Default::default(),
			description: Default::default(),
			is_favorite: Default::default(),
			is_enabled: Default::default(),
			run_at: Default::default(),
			created_at: Default::default(),
			updated_at: Default::default(),
		}
	}
}

#[derive(Clone, CandidType, Serialize, Deserialize, Debug)]
pub struct PostCircuit {
	pub name: String,
	pub description: Option<String>,
}
