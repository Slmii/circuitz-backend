use std::borrow::Cow;
use candid::{ CandidType, Principal, Decode, Encode };
use ic_stable_structures::{ storable::Bound, Storable };
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CircuitKey {
	pub id: u32,
	pub owner: String,
}

impl Storable for CircuitKey {
	fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
		Cow::Owned(Encode!(self).unwrap())
	}

	fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
		Decode!(bytes.as_ref(), Self).unwrap()
	}

	const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, Debug, CandidType, Deserialize)]
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

impl Storable for Circuit {
	fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
		Cow::Owned(Encode!(self).unwrap())
	}

	fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
		Decode!(bytes.as_ref(), Self).unwrap()
	}

	const BOUND: Bound = Bound::Unbounded;
}

impl Default for Circuit {
	fn default() -> Self {
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
