use candid::Principal;
use crate::{ lib::types::trace::Trace, storage::canister_storage::TRACES };

pub struct TracesStore;

impl TracesStore {
	/// Get traces by circuit ID.
	///
	/// # Arguments
	/// - `circuit_id` - Circuit ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Vec<Node>` - Nodes
	pub fn get_circuit_traces(circuit_id: u32, caller_principal: Principal) -> Vec<Trace> {
		TRACES.with(|traces| {
			let traces = traces.borrow();

			// Get circuit's traces
			let circuit_traces = traces
				.iter()
				.filter(|(key, _)| key.circuit_id == circuit_id && key.owner == caller_principal.to_string())
				.map(|(_, node)| node.clone())
				.collect::<Vec<Trace>>();

			circuit_traces
		})
	}
}
