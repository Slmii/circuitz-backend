use ic_cdk::caller;
use ic_cdk_macros::{ query, update };
use lib::{ types::{ circuit::{ Circuit, PostCircuit }, api_error::ApiError }, utils::validate_anonymous };
use crate::circuits_store::CircuitsStore;

#[query]
fn get_user_circuits() -> Result<Vec<Circuit>, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(CircuitsStore::get_user_circuits(caller_principal)),
		Err(err) => Err(err),
	}
}

#[update]
fn add_circuit(post_circuit: PostCircuit) -> Result<Circuit, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(CircuitsStore::add_circuit(post_circuit, caller_principal)),
		Err(err) => Err(err),
	}
}

#[test]
fn generate_candid() {
	use candid::export_service;
	use lib::save_candid;
	export_service!();

	save_candid::save_candid(__export_service(), "circuits".to_string());
}
