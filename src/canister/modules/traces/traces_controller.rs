use ic_cdk::{ caller, query };
use lib::{ types::{ api_error::ApiError, trace::Trace }, utils::validate::validate_anonymous };
use super::traces_store::TracesStore;

#[query]
fn get_circuit_traces(circuit_id: u32) -> Result<Vec<Trace>, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(TracesStore::get_circuit_traces(circuit_id, caller_principal)),
		Err(err) => Err(err),
	}
}
