use candid::Principal;
use ic_cdk::api::time;
use ic_stable_structures::{
	memory_manager::{ VirtualMemory, MemoryManager, MemoryId },
	DefaultMemoryImpl,
	StableBTreeMap,
};
use lib::types::{ user::User, api_error::ApiError };
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

pub struct UsersStore {}

thread_local! {
	pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
		MemoryManager::init(DefaultMemoryImpl::default())
	);

	pub static USERS: RefCell<StableBTreeMap<String, User, Memory>> = RefCell::new(
		StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
	);
}

impl UsersStore {
	/// Get users.
	///
	/// # Returns
	/// - `User` - User
	pub fn get_users() -> Vec<User> {
		USERS.with(|users| {
			let users = users.borrow();

			users
				.iter()
				.map(|(_, user)| user.clone())
				.collect()
		})
	}

	/// Get user by principal.
	///
	/// # Arguments
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `User` - User
	pub fn get_user(caller_principal: Principal) -> Result<User, ApiError> {
		USERS.with(|state| {
			let state = state.borrow();

			let opt_user = state.get(&caller_principal.to_string());
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
		let user = USERS.with(|state| {
			let mut state = state.borrow_mut();

			if state.contains_key(&caller_principal.to_string()) {
				return Err(ApiError::AlreadyExists("USER_EXISTS".to_string()));
			}

			let user_to_add = User {
				user_id: caller_principal,
				username,
				created_at: time(),
				circuits: vec![],
			};

			state.insert(caller_principal.to_string(), user_to_add.clone());

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
