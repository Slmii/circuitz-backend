use ic_cdk::caller;
use ic_cdk_macros::query;
use lib::{ types::{ trace::Trace, api_error::ApiError }, utils::validate_anonymous };
use crate::traces_store::TracesStore;

#[query]
fn get_circuit_traces(circuit_id: u32) -> Result<Vec<Trace>, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(TracesStore::get_circuit_traces(circuit_id, caller_principal)),
		Err(err) => Err(err),
	}
}

#[test]
fn generate_candid() {
	use candid::export_service;
	use lib::save_candid;
	export_service!();

	save_candid::save_candid(__export_service(), "traces".to_string());
}
