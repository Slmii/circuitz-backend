use candid::{ Principal, encode_args };
use ic_cdk::{ id, api::{ time, call::call_raw } };
use ic_stable_structures::{
	memory_manager::{ MemoryManager, MemoryId },
	DefaultMemoryImpl,
	StableCell,
	StableBTreeMap,
};
use lib::types::{
	node::{ Node, NodeType },
	api_error::ApiError,
	memory::Memory,
	node_type_lookup::{ LookupCanister, Arg },
	tuple_variant::TupleVariant,
};
use std::cell::RefCell;

pub struct NodesStore {}

thread_local! {
	pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
		MemoryManager::init(DefaultMemoryImpl::default())
	);

	pub static NODES: RefCell<StableBTreeMap<u32, Node, Memory>> = RefCell::new(
		StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
	);

	pub static CANISTER_OWNER: RefCell<StableCell<String, Memory>> = RefCell::new(
		StableCell::init(
			MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
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
	pub fn get_circuit_nodes(
		circuit_id: u32,
		_caller_principal: Principal
	) -> Result<(Principal, Vec<Node>), ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let nodes = nodes.borrow();

			// if caller_principal.to_string() != canister_owner {
			// 	// If the caller is not the canister owner, return an error
			// 	return Err(ApiError::NotFound("UNAUTHORIZED".to_string()));
			// }

			// Get circuit's nodes
			let circuit_nodes = nodes
				.iter()
				.filter(|(_, node)| node.circuit_id == circuit_id)
				.map(|(_, node)| node.clone())
				.collect::<Vec<Node>>();

			Ok((id(), circuit_nodes))
		})
	}

	/// Add a node to a circuit.
	///
	/// # Arguments
	/// - `circuit_id` - Circuit ID
	/// - `data` - Node data
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Node
	pub fn add_node(circuit_id: u32, data: NodeType, caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let mut nodes = nodes.borrow_mut();

			// if caller_principal.to_string() != canister_owner {
			// 	// If the caller is not the canister owner, return an error
			// 	return Err(ApiError::NotFound("UNAUTHORIZED".to_string()));
			// }

			let node_id = (nodes.len() as u32) + 1;

			let new_node = Node {
				id: node_id,
				circuit_id,
				user_id: caller_principal,
				is_enabled: true,
				is_error: false,
				is_running: false,
				node_type: data,
				order: node_id, // node_id is the order
				pin: vec![],
				created_at: time(),
				updated_at: time(),
			};

			// Add new node
			nodes.insert(node_id, new_node.clone());

			Ok(new_node)
		})
	}

	/// Edit a node.
	///
	/// # Arguments
	/// - `node_id` - Node ID
	/// - `data` - Node data
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Node
	pub fn edit_node(node_id: u32, data: NodeType, _caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let mut nodes = nodes.borrow_mut();

			// if caller_principal.to_string() != canister_owner {
			// 	// If the caller is not the canister owner, return an error
			// 	return Err(ApiError::NotFound("UNAUTHORIZED".to_string()));
			// }

			let node = nodes.get(&node_id);

			if node.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			let mut node = node.unwrap().clone();

			// Mutate values
			node.node_type = data;
			node.updated_at = time();

			// Add new node or overwrite existing one
			nodes.insert(node_id, node.clone());

			Ok(node)
		})
	}

	/// Preview lookup canister request
	///
	/// # Arguments
	/// - `data` - LookupCanister
	///
	/// # Returns
	/// - `Unknown` - Unknown data from the canister
	pub async fn preview_lookup_request(data: LookupCanister) -> Result<Vec<u8>, ApiError> {
		let to_tuple = Self::vec_to_tuple(data.args);

		match to_tuple {
			Ok(tuple) => {
				match tuple {
					TupleVariant::One(variant) => {
						match variant {
							Arg::Number(number) => {
								let args_raw = encode_args((number,)).unwrap();
								let response = call_raw(data.canister, &data.method, args_raw, 0).await;

								// let response: Result<(String,), (RejectionCode, String)> = call::call(
								// 	data.canister,
								// 	&data.method,
								// 	(number,)
								// ).await;

								match response {
									Ok(bytes) => Ok(bytes),
									Err((_, message)) => Err(ApiError::InterCanister(message)),
								}
							}
							_ => {
								return Err(ApiError::InterCanister("Failed to convert to tuple".to_string()));
							}
						}
					}
					_ => {
						return Err(ApiError::InterCanister("Failed to convert to tuple".to_string()));
					}
				}
			}
			Err(_) => Err(ApiError::InterCanister("Failed to convert to tuple".to_string())),
		}
	}

	fn vec_to_tuple<T: Clone>(v: Vec<T>) -> Result<TupleVariant<T>, &'static str> {
		match v.len() {
			0 => Err("Vec is empty"),
			1 => Ok(TupleVariant::One(v[0].clone())),
			2 => Ok(TupleVariant::Two(v[0].clone(), v[1].clone())),
			3 => Ok(TupleVariant::Three(v[0].clone(), v[1].clone(), v[2].clone())),
			4 => Ok(TupleVariant::Four(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone())),
			5 => Ok(TupleVariant::Five(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone(), v[4].clone())),
			6 =>
				Ok(
					TupleVariant::Six(
						v[0].clone(),
						v[1].clone(),
						v[2].clone(),
						v[3].clone(),
						v[4].clone(),
						v[5].clone()
					)
				),
			7 =>
				Ok(
					TupleVariant::Seven(
						v[0].clone(),
						v[1].clone(),
						v[2].clone(),
						v[3].clone(),
						v[4].clone(),
						v[5].clone(),
						v[6].clone()
					)
				),
			8 =>
				Ok(
					TupleVariant::Eight(
						v[0].clone(),
						v[1].clone(),
						v[2].clone(),
						v[3].clone(),
						v[4].clone(),
						v[5].clone(),
						v[6].clone(),
						v[7].clone()
					)
				),
			9 =>
				Ok(
					TupleVariant::Nine(
						v[0].clone(),
						v[1].clone(),
						v[2].clone(),
						v[3].clone(),
						v[4].clone(),
						v[5].clone(),
						v[6].clone(),
						v[7].clone(),
						v[8].clone()
					)
				),
			10 =>
				Ok(
					TupleVariant::Ten(
						v[0].clone(),
						v[1].clone(),
						v[2].clone(),
						v[3].clone(),
						v[4].clone(),
						v[5].clone(),
						v[6].clone(),
						v[7].clone(),
						v[8].clone(),
						v[9].clone()
					)
				),
			// You can continue this pattern for as many sizes as you wish to handle
			_ => Err("Vec is too large to convert to a known tuple size"),
		}
	}

	// pub async fn preview_lookup_request<T: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
	// 	data: LookupCanister,
	// 	args: T
	// ) -> Result<R, ApiError> {
	// 	let args_raw = encode_args(args).expect("Failed to encode arguments.");
	// 	let fut = call_raw(data.canister, &data.method, args_raw, 0).await;

	// 	match fut {
	// 		Ok(bytes) => {
	// 			let decoded = decode_args::<R>(&bytes);

	// 			match decoded {
	// 				Ok(response) => Ok(response),
	// 				Err(err) => Err(ApiError::InterCanister(err.to_string())),
	// 			}
	// 		}
	// 		Err(_) => {
	// 			return Err(ApiError::InterCanister("Failed to encode bytes".to_string()));
	// 		}
	// 	}
	// }
}
