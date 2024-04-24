use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };
use crate::impl_storable_for;

impl_storable_for!(Circuit);
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Circuit {
	pub id: u32,
	pub user_id: Principal,
	pub node_canister_id: Principal,
	pub name: String,
	pub description: Option<String>,
	pub is_favorite: bool,
	pub is_enabled: bool,
	pub is_running: bool,
	pub run_at: Option<u64>,
	pub created_at: u64,
	pub updated_at: u64,
}

impl Circuit {
	pub fn default() -> Self {
		Self {
			id: Default::default(),
			user_id: Principal::anonymous(),
			node_canister_id: Principal::anonymous(),
			name: Default::default(),
			description: Default::default(),
			is_favorite: Default::default(),
			is_enabled: Default::default(),
			is_running: Default::default(),
			run_at: Default::default(),
			created_at: Default::default(),
			updated_at: Default::default(),
		}
	}
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PostCircuit {
	pub name: String,
	pub description: Option<String>,
}
