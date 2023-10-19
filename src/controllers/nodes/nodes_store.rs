use candid::{ Principal, ser::IDLBuilder, IDLArgs, IDLValue, Decode, parser::types::IDLType, IDLProg };
use ic_cdk::{ id, api::{ time, call::call_raw } };
use ic_stable_structures::{
	memory_manager::{ MemoryManager, MemoryId },
	DefaultMemoryImpl,
	StableCell,
	StableBTreeMap,
};
use lib::{
	types::{
		node::{ Node, NodeType },
		api_error::ApiError,
		memory::Memory,
		node_type_lookup::{ LookupCanister, Arg },
		tuple_variant::TupleVariant,
	},
	conversion::{ Idl2JsonOptions, idl2json_with_weak_names, idl_prog, idl2json },
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
	pub async fn preview_lookup_request(data: LookupCanister) -> Result<String, ApiError> {
		let to_tuple = Self::vec_to_tuple(data.args);

		match to_tuple {
			Ok(tuple) => {
				match tuple {
					TupleVariant::Zero => {
						let bytes = IDLBuilder::new().serialize_to_vec().unwrap();

						let response = call_raw(data.canister, &data.method, bytes, 0).await.unwrap();
						let decoded = IDLArgs::from_bytes(&response).unwrap();
						let output = decoded.to_string();

						Ok(output)
					}
					TupleVariant::One(variant) => {
						match variant {
							Arg::Number(number) => {
								let bytes = IDLBuilder::new()
									.value_arg(&IDLValue::Nat32(number))
									.unwrap()
									.serialize_to_vec()
									.unwrap();

								// let mut env = TypeEnv::new();
								// let metadata: Vec<u8> = ic_agent::agent::Agent
								// 	::read_state_canister_metadata(
								// 		&AgentBuilder::default().build().unwrap(),
								// 		data.canister,
								// 		&"candid:service".to_string()
								// 	).await
								// 	.unwrap();

								// let prog = IDLProg::from_str(&metadata).expect("Failed to parse did");

								// let idl_type = polyfill::idl_prog
								// 	::get_type(&prog, "candid:service")
								// 	.expect("Failed to get idltype");
								// let idl_type = IDLType::OptT(Box::new(idl_type));

								let encoded = call_raw(data.canister, &data.method, bytes, 0).await.unwrap();
								let idl_value = Decode!(&encoded[..], IDLValue).expect("Failed to decode IDLValue");

								let idl_json = idl2json(&idl_value, &Idl2JsonOptions::default());

								let idl_str =
									r#"
									type ApiError = variant {
										NotFound : text;
										Unauthorized : text;
										AlreadyExists : text;
										InterCanister : text;
									};
									type Circuit = record {
										id : nat32;
										updated_at : nat64;
										run_at : opt nat64;
										name : text;
										is_enabled : bool;
										description : opt text;
										created_at : nat64;
										user_id : principal;
										is_favorite : bool;
										node_canister_id : principal;
										is_running : bool;
									};
									type PostCircuit = record { name : text; description : opt text };
									type Result = variant { Ok : Circuit; Err : ApiError };
									type Result_1 = variant { Ok : principal; Err : ApiError };
									type Result_2 = variant { Ok : vec Circuit; Err : ApiError };
									service : {
										add_circuit : (PostCircuit) -> (Result);
										disable_circuit : (nat32) -> (Result);
										edit_circuit : (nat32, PostCircuit) -> (Result);
										enable_circuit : (nat32) -> (Result);
										get_circuit : (nat32) -> (Result) query;
										get_circuits : () -> (vec Circuit) query;
										get_node_canister_id : (nat32) -> (Result_1) query;
										get_user_circuits : () -> (Result_2) query;
									}
								"#;

								let prog = idl_str.parse().expect("Failed to parse did");
								let idl_type = idl_prog::get_type(&prog, "Circuit").expect("Failed to get idltype");
								let idl_type = IDLType::OptT(Box::new(idl_type));
								let idl_json_weak_names = idl2json_with_weak_names(
									&idl_value,
									&idl_type,
									&Idl2JsonOptions::default()
								);
								// // let json_string = serde_json::to_string(&idl_json).unwrap();
								let json_string = serde_json::to_string(&idl_json_weak_names).unwrap();

								Ok(json_string)
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
			0 => Ok(TupleVariant::Zero),
			1 => Ok(TupleVariant::One(v[0].clone())),
			2 => Ok(TupleVariant::Two(v[0].clone(), v[1].clone())),
			3 => Ok(TupleVariant::Three(v[0].clone(), v[1].clone(), v[2].clone())),
			4 => Ok(TupleVariant::Four(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone())),
			5 => Ok(TupleVariant::Five(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone(), v[4].clone())),
			_ => Err("Vec is too large to convert to a known tuple size"),
		}
	}
}
