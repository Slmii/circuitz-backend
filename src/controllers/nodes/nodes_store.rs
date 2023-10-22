use candid::Principal;
use ic_cdk::{
	api::{
		management_canister::http_request::{
			http_request,
			CanisterHttpRequestArgument,
			HttpHeader,
			HttpMethod,
			TransformContext,
			TransformFunc,
		},
		time,
	},
	id,
};

use ic_stable_structures::{
	memory_manager::{ MemoryManager, MemoryId },
	DefaultMemoryImpl,
	StableCell,
	StableBTreeMap,
};
use lib::{
	types::{ node::{ Node, NodeType, LookupCanister }, memory::Memory, api_error::ApiError },
	utils::idempotency::generate_idempotency_key,
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
	pub async fn preview_lookup_request(
		data: LookupCanister,
		_caller_principal: Principal
	) -> Result<String, ApiError> {
		// if caller_principal.to_string() != canister_owner {
		// 	// If the caller is not the canister owner, return an error
		// 	return Err(ApiError::NotFound("UNAUTHORIZED".to_string()));
		// }

		// Setup the URL
		let host = "circuitz-node-modlx.ondigitalocean.app";
		let url = "https://circuitz-node-modlx.ondigitalocean.app/icc";

		// Prepare headers for the system http_request call
		let request_headers = vec![
			HttpHeader {
				name: "Host".to_string(),
				value: format!("{host}:443"),
			},
			HttpHeader {
				name: "User-Agent".to_string(),
				value: "demo_HTTP_POST_canister".to_string(),
			},
			//For the purposes of this exercise, Idempotency-Key" is hard coded, but in practice
			//it should be generated via code and unique to each POST request. Common to create helper methods for this
			HttpHeader {
				name: "Idempotency-Key".to_string(),
				value: generate_idempotency_key().await.unwrap(),
			},
			HttpHeader {
				name: "Content-Type".to_string(),
				value: "application/json".to_string(),
			}
		];

		let request_body_value =
			serde_json::json!({
			"canisterId": data.canister,
			"methodName": data.method,
			"args": data.args,
		});
		let request_body: Option<Vec<u8>> = Some(request_body_value.to_string().into_bytes());

		let request = CanisterHttpRequestArgument {
			url: url.to_string(),
			max_response_bytes: None, //optional for request
			method: HttpMethod::POST,
			headers: request_headers,
			body: request_body,
			transform: Some(TransformContext {
				function: TransformFunc(candid::Func {
					method: "transform".into(),
					principal: ic_cdk::id(),
				}),
				context: vec![],
			}),
		};

		match http_request(request, data.cycles).await {
			Ok((response,)) => {
				// if successful, `HttpResponse` has this structure:
				// pub struct HttpResponse {
				//     pub status: Nat,
				//     pub headers: Vec<HttpHeader>,
				//     pub body: Vec<u8>,
				// }

				// We need to decode that Vec<u8> that is the body into readable text.
				// To do this, we:
				//  1. Call `String::from_utf8()` on response.body
				//  3. We use a switch to explicitly call out both cases of decoding the Blob into ?Text
				let str_body = String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");

				// The API response will looks like this:
				// { successful: true }

				// Return the body as a string and end the method
				let result: String = format!("{}. See more info of the request sent at: {}/inspect", str_body, url);

				return Ok(result);
			}
			Err((r, m)) => {
				let message = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

				//Return the error as a string and end the method
				return Err(ApiError::InterCanister(message));
			}
		}
	}
}
