use candid::Principal;
use ic_cdk::id;
use ic_stable_structures::{
	memory_manager::{ VirtualMemory, MemoryManager, MemoryId },
	DefaultMemoryImpl,
	StableCell,
	StableVec,
};
use lib::types::{ node::Node, api_error::ApiError };
use std::cell::RefCell;
type Memory = VirtualMemory<DefaultMemoryImpl>;

pub struct NodesStore {}

thread_local! {
	pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
		MemoryManager::init(DefaultMemoryImpl::default())
	);

	// Node ID
	pub static NODE_ID: RefCell<StableCell<u32, Memory>> = RefCell::new(
		StableCell::init(
			MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
			0
		).expect("Failed to initialize NODE_ID")
	);

	// Node ID -> Node
	pub static NODES: RefCell<StableVec<Node, Memory>> = RefCell::new(
		StableVec::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))).expect("Failed to initialize NODES")
	);

	// Canister owner
	pub static CANISTER_OWNER: RefCell<StableCell<String, Memory>> = RefCell::new(
		StableCell::init(
			MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
			String::default()
		).expect("Failed to initialize CANISTER_OWNER")
	);
}

impl NodesStore {
	/// Get nodes by circuit ID.
	///
	/// # Arguments
	/// - `circuit_id` - Circuit ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Vec<Node>` - Nodes
	pub fn get_circuit_nodes(circuit_id: u32, caller_principal: Principal) -> Result<(Principal, Vec<Node>), ApiError> {
		let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let nodes = nodes.borrow();

			if caller_principal.to_string() != canister_owner {
				// If the caller is not the canister owner, return an error
				return Err(ApiError::NotFound("UNAUTHORIZED".to_string()));
			}

			// Get circuit's nodes
			let circuit_nodes = nodes
				.iter()
				.filter(|node| node.circuit_id == circuit_id)
				.collect::<Vec<Node>>();

			Ok((id(), circuit_nodes))
		})
	}
}
