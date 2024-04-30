use ic_cdk::{ caller, query, update };
use lib::{ types::{ api_error::ApiError, user::User }, utils::validate::{ validate_admin, validate_anonymous } };

use super::users_store::UsersStore;

#[query]
fn get_users() -> Result<Vec<User>, ApiError> {
	match validate_admin(&caller()) {
		Ok(_) => Ok(UsersStore::get_users()),
		Err(err) => Err(err),
	}
}

#[query]
fn get_user() -> Result<User, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => UsersStore::get_user(caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn create_user(username: Option<String>) -> Result<User, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => UsersStore::create_user(caller_principal, username),
		Err(err) => Err(err),
	}
}
