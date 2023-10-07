use ic_cdk::{ storage, caller };
use ic_cdk_macros::{ post_upgrade, query, update, pre_upgrade, export_candid };
use lib::{ types::{ circuit::{ Circuit, PostCircuit }, api_error::ApiError }, utils::validate_anonymous };
use crate::circuits_store::{ STATE, CircuitsStore };

#[pre_upgrade]
fn pre_upgrade() {
	STATE.with(|state| storage::stable_save((state,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
	let (old_state,): (CircuitsStore,) = storage::stable_restore().unwrap();
	STATE.with(|state| {
		*state.borrow_mut() = old_state;
	});
}

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

export_candid!();
