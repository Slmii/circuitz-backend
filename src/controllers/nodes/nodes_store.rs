use candid::{ CandidType, Deserialize, Principal };
use lib::types::node::Node;
use std::{ cell::RefCell, collections::HashMap };

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct NodesStore {
	// Increment of node IDs
	pub node_id: u32,
	// All nodes in the system. u32 = node_id
	pub nodes: HashMap<u32, Node>,
	// Caller's nodes. <(u32 = circuit_id, Principal = caller), u32 = node_id>
	pub user_nodes: HashMap<(u32, Principal), Vec<u32>>,
}

thread_local! {
	pub static STATE: RefCell<NodesStore> = RefCell::new(NodesStore::default());
}

impl NodesStore {
	/// Get nodes by principal.
	///
	/// # Arguments
	/// - `circuit_id` - Circuit ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Vec<Node>` - Nodes
	pub fn get_circuit_nodes(circuit_id: u32, caller_principal: Principal) -> Vec<Node> {
		STATE.with(|state| {
			let state = state.borrow();

			// Get circuit's nodes
			let node_ids_by_principal = state.user_nodes
				.get(&(circuit_id, caller_principal))
				.cloned()
				.unwrap_or_default();

			// Loop through all nodes and check if the node_id contains in user's node list
			state.nodes
				.values()
				.filter(|node| node_ids_by_principal.contains(&node.id))
				.cloned()
				.collect()
		})
	}
}
