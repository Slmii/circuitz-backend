use candid::Principal;
use ic_cdk::caller;
use ic_cdk_macros::query;
use lib::{ types::{ node::Node, api_error::ApiError }, utils::validate_anonymous };
use crate::nodes_store::NodesStore;

#[query]
fn get_circuit_nodes(circuit_id: u32) -> Result<(Principal, Vec<Node>), ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::get_circuit_nodes(circuit_id, caller_principal),
		Err(err) => Err(err),
	}
}

// #[init]
// #[candid_method(init)]
// fn init(canister_owner: Option<Principal>) {
// 	// TODO: add controller and owner to created caniser
// }

#[test]
fn generate_candid() {
	use candid::export_service;
	use lib::save_candid;
	export_service!();

	save_candid::save_candid(__export_service(), "nodes".to_string());
}
