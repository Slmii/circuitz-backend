use std::borrow::Cow;
use candid::{ CandidType, Deserialize, Principal, Decode, Encode };
use ic_stable_structures::{ storable::Bound, Storable };

#[derive(CandidType, Clone, Deserialize)]
pub struct User {
	pub user_id: Principal,
	pub username: Option<String>,
	pub created_at: u64,
	pub circuits: Vec<u32>,
}

impl Storable for User {
	fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
		Cow::Owned(Encode!(self).unwrap())
	}

	fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
		Decode!(bytes.as_ref(), Self).unwrap()
	}

	const BOUND: Bound = Bound::Unbounded;
}

impl Default for User {
	fn default() -> Self {
		Self {
			user_id: Principal::anonymous(),
			username: Default::default(),
			created_at: Default::default(),
			circuits: Default::default(),
		}
	}
}
