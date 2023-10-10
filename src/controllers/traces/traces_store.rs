use candid::Principal;
use ic_stable_structures::{ memory_manager::{ MemoryManager, MemoryId }, DefaultMemoryImpl, StableBTreeMap };
use lib::types::{ trace::{ Trace, TraceKey }, memory::Memory };
use std::cell::RefCell;

pub struct TracesStore {}

thread_local! {
	pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
		MemoryManager::init(DefaultMemoryImpl::default())
	);

	pub static TRACES: RefCell<StableBTreeMap<TraceKey, Trace, Memory>> = RefCell::new(
		StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
	);
}

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
