use candid::Principal;
use ic_cdk::caller;
use ic_cdk_macros::{ query, update };
use lib::{ types::{ circuit::{ Circuit, PostCircuit }, api_error::ApiError }, utils::validate::validate_anonymous };
use crate::circuits_store::CircuitsStore;

#[query]
fn get_circuit(circuit_id: u32) -> Result<Circuit, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => CircuitsStore::get_circuit(circuit_id, caller_principal),
		Err(err) => Err(err),
	}
}

#[query]
fn get_circuits() -> Vec<Circuit> {
	CircuitsStore::get_circuits()
}

#[query]
fn get_user_circuits() -> Result<Vec<Circuit>, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(CircuitsStore::get_user_circuits(caller_principal)),
		Err(err) => Err(err),
	}
}

#[query]
fn get_node_canister_id(circuit_id: u32) -> Result<Principal, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => CircuitsStore::get_node_canister_id(circuit_id, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn enable_circuit(circuit_id: u32) -> Result<Circuit, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => CircuitsStore::toggle_circuit(circuit_id, true, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn disable_circuit(circuit_id: u32) -> Result<Circuit, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => CircuitsStore::toggle_circuit(circuit_id, false, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn add_circuit(data: PostCircuit) -> Result<Circuit, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(CircuitsStore::add_circuit(data, caller_principal)),
		Err(err) => Err(err),
	}
}

#[update]
fn edit_circuit(circuit_id: u32, data: PostCircuit) -> Result<Circuit, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => CircuitsStore::edit_circuit(circuit_id, data, caller_principal),
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
