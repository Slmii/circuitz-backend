use candid::{ CandidType, Deserialize, Principal };
use ic_cdk::api::time;
use lib::types::{ user::User, api_error::ApiError };
use std::{ cell::RefCell, collections::HashMap };

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct UsersStore {
	pub users: HashMap<Principal, User>,
}

thread_local! {
	pub static STATE: RefCell<UsersStore> = RefCell::new(UsersStore::default());
}

impl UsersStore {
	/// Get user by principal.
	///
	/// # Arguments
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `User` - User
	pub fn get_user(caller_principal: Principal) -> Result<User, ApiError> {
		STATE.with(|state| {
			let state = state.borrow();

			let opt_user = state.users.get(&caller_principal);
			opt_user.map_or(Err(ApiError::NotFound("USER_NOT_FOUND".to_string())), |user| Ok(user.clone()))
		})
	}

	/// Create user.
	///
	/// # Arguments
	/// - `caller_principal` - Principal of the caller
	/// - `username` - Username
	///
	/// # Returns
	/// - `User` - User
	pub async fn create_user(caller_principal: Principal, username: Option<String>) -> Result<User, ApiError> {
		let user = STATE.with(|state| {
			let mut state = state.borrow_mut();

			if state.users.contains_key(&caller_principal) {
				return Err(ApiError::AlreadyExists("USER_EXISTS".to_string()));
			}

			let user_to_add = User {
				user_id: caller_principal,
				username,
				created_at: time(),
				circuits: vec![],
			};

			state.users.insert(caller_principal, user_to_add.clone());

			Ok(user_to_add.clone())
		});

		match user {
			// If user is created successfully
			Ok(user) => Ok(user),
			// If user creation failed
			Err(error) => Err(error),
		}
	}
}
