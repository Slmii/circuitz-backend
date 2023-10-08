use crate::users_store::{ UsersStore, STATE };
use ic_cdk::{ caller, storage };
use ic_cdk_macros::{ post_upgrade, query, update, pre_upgrade };
use lib::{ types::{ api_error::ApiError, user::User }, utils::validate_anonymous };

#[pre_upgrade]
fn pre_upgrade() {
	STATE.with(|state| storage::stable_save((state,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
	let (old_state,): (UsersStore,) = storage::stable_restore().unwrap();
	STATE.with(|state| {
		*state.borrow_mut() = old_state;
	});
}

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
