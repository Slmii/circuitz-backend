use ic_cdk::{ storage, caller };
use ic_cdk_macros::{ post_upgrade, query, pre_upgrade };
use lib::{ types::{ node::Node, api_error::ApiError }, utils::validate_anonymous };
use crate::nodes_store::{ STATE, NodesStore };

#[pre_upgrade]
fn pre_upgrade() {
	STATE.with(|state| storage::stable_save((state,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
	let (old_state,): (NodesStore,) = storage::stable_restore().unwrap();
	STATE.with(|state| {
		*state.borrow_mut() = old_state;
	});
}

#[query]
fn get_circuit_nodes(circuit_id: u32) -> Result<Vec<Node>, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(NodesStore::get_circuit_nodes(circuit_id, caller_principal)),
		Err(err) => Err(err),
	}
}

#[test]
fn generate_candid() {
	use candid::export_service;
	use lib::save_candid;
	export_service!();

	save_candid::save_candid(__export_service(), "nodes".to_string());
}
