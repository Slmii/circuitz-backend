use candid::Principal;
use ic_cdk::{ caller, query, update };
use lib::{ types::{ api_error::ApiError, circuit::{ Circuit, PostCircuit } }, utils::validate::validate_anonymous };
use super::circuits_store::CircuitsStore;

#[query]
fn get_circuit(circuit_id: u32) -> Result<Circuit, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => CircuitsStore::get_circuit(circuit_id, caller_principal),
		Err(err) => Err(err),
	}
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
