use crate::users_store::UsersStore;
use ic_cdk::caller;
use ic_cdk_macros::{ query, update };
use lib::{ types::{ api_error::ApiError, user::User }, utils::validate_anonymous };

#[query]
fn get_user() -> Result<User, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => UsersStore::get_user(caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
async fn create_user(username: Option<String>) -> Result<User, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => UsersStore::create_user(caller_principal, username).await,
		Err(err) => Err(err),
	}
}

#[test]
fn generate_candid() {
	use candid::export_service;
	use lib::save_candid;
	export_service!();

	save_candid::save_candid(__export_service(), "users".to_string());
}
