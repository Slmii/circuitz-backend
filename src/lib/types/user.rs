use candid::{ CandidType, Deserialize, Principal };

#[derive(CandidType, Clone, Deserialize)]
pub struct User {
	pub user_id: Principal,
	pub username: Option<String>,
	pub created_at: u64,
	pub circuits: Vec<u32>,
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
