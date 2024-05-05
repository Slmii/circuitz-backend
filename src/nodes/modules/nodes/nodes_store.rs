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
use lib::{
	types::{ api_error::ApiError, node::{ LookupCanister, LookupHttpRequest, Node, NodeType, Pin, PinType } },
	utils::idempotency::generate_idempotency_key,
};
use serde_json::Value;
use url::Url;

use crate::canister_storage::NODES;

// TODO: add node owner check to all calls
// thread_local! {
// 	pub static CANISTER_OWNER: RefCell<StableCell<String, Memory>> = RefCell::new(
// 		StableCell::init(
// 			MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
// 			String::default()
// 		).expect("Failed to initialize CANISTER_OWNER")
// 	);
// }
// if caller_principal.to_string() != canister_owner {
// 	// If the caller is not the canister owner, return an error
// 	return Err(ApiError::NotFound("UNAUTHORIZED".to_string()));
// }

pub struct NodesStore;

impl NodesStore {
	/// Get node by ID.
	///
	/// # Arguments
	/// - `node_id` - Node ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Node
	pub fn get_circuit_node(node_id: u32, _caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let nodes = nodes.borrow();

			// Get circuit's nodes
			let circuit_node = nodes.get(&node_id);

			if let Some(node) = circuit_node {
				Ok(node.clone())
			} else {
				Err(ApiError::NotFound("NOT FOUND".to_string()))
			}
		})
	}

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

			// Get circuit's nodes
			let circuit_nodes = nodes
				.iter()
				.filter(|(_, node)| node.circuit_id == circuit_id)
				.map(|(_, node)| node.clone())
				.collect::<Vec<Node>>();

			Ok((id(), circuit_nodes))
		})
	}

	/// Delete a node from a circuit.
	///
	/// # Arguments
	/// - `node_id` - Circuit ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Node
	pub fn delete_node(node_id: u32, _caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let mut nodes = nodes.borrow_mut();

			let node = nodes.get(&node_id);
			if node.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			let node = node.unwrap().clone();

			// Remove node
			nodes.remove(&node.id);

			Ok(node)
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
				pins: vec![],
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

	/// Edit a node's order.
	///
	/// # Arguments
	/// - `node_id` - Node ID
	/// - `order` - Node order
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Node
	pub fn edit_order(node_id: u32, order: u32, _caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let mut nodes = nodes.borrow_mut();

			let node = nodes.get(&node_id);
			if node.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			let mut node = node.unwrap().clone();

			// Mutate values
			node.order = order;
			node.updated_at = time();

			// Add new node or overwrite existing one
			nodes.insert(node_id, node.clone());

			Ok(node)
		})
	}

	/// Toggle enable/disable node.
	///
	/// # Arguments
	/// - `node_id` - Circuit ID
	/// - `enabled` - Enabled or disabled
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Enabled node
	pub fn toggle_node(node_id: u32, enabled: bool, _caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let mut nodes = nodes.borrow_mut();

			let node = nodes.get(&node_id);
			if node.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			let mut node = node.unwrap().clone();

			// Mutate values
			node.is_enabled = enabled;
			node.updated_at = time();

			// Add new node or overwrite existing one
			nodes.insert(node_id, node.clone());

			Ok(node)
		})
	}

	/// Add pin to node.
	///
	/// # Arguments
	/// - `node_id` - Circuit ID
	/// - `data` - Pin data
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Node
	pub fn add_pin(node_id: u32, data: Pin, _caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let mut nodes = nodes.borrow_mut();

			let node = nodes.get(&node_id);
			if node.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			let mut node = node.unwrap().clone();

			// Check if the pin already exists
			let pin_index_opt = node.pins.iter().position(|pin| pin.pin_type == data.pin_type);
			match pin_index_opt {
				Some(_) => {
					return Err(ApiError::NotFound("ALREADY EXISTS".to_string()));
				}
				None => {
					// Mutate values
					node.pins.push(data);

					// Add new node or overwrite existing one
					nodes.insert(node_id, node.clone());

					Ok(node)
				}
			}
		})
	}

	/// Edit pin of node.
	///
	/// # Arguments
	/// - `node_id` - Circuit ID
	/// - `data` - Pin data
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Node
	pub fn edit_pin(node_id: u32, data: Pin, _caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let mut nodes = nodes.borrow_mut();

			// Ensure the node exists
			let node_opt = nodes.get(&node_id);
			let mut node = match node_opt {
				Some(n) => n,
				None => {
					return Err(ApiError::NotFound("NOT FOUND".to_string()));
				}
			};

			// Find the index of the pin to edit based on PinType
			let pin_index_opt = node.pins.iter().position(|pin| {
				match (&pin.pin_type, &data.pin_type) {
					(PinType::PrePin(_), PinType::PrePin(_)) => true,
					(PinType::PostPin(_), PinType::PostPin(_)) => true,
					(PinType::MapperPin(_), PinType::MapperPin(_)) => true,
					(PinType::FilterPin(_), PinType::FilterPin(_)) => true,
					(PinType::LookupTransformPin(_), PinType::LookupTransformPin(_)) => true,
					(PinType::LookupFilterPin(_), PinType::LookupFilterPin(_)) => true,
					_ => false,
				}
			});

			let pin_index = match pin_index_opt {
				Some(idx) => idx,
				None => {
					return Err(ApiError::NotFound("NODE INDEX NOT FOUND".to_string()));
				}
			};

			// Update the pin
			node.pins[pin_index] = data;
			nodes.insert(node_id, node.clone());

			Ok(node)
		})
	}

	/// Delete pin from node.
	///
	/// # Arguments
	/// - `node_id` - Circuit ID
	/// - `pin_id` - Pin ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Node` - Node
	pub fn delete_pin(node_id: u32, data: Pin, _caller_principal: Principal) -> Result<Node, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		NODES.with(|nodes| {
			let mut nodes = nodes.borrow_mut();

			// Ensure the node exists
			let node_opt = nodes.get(&node_id);
			let mut node = match node_opt {
				Some(n) => n,
				None => {
					return Err(ApiError::NotFound("NOT FOUND".to_string()));
				}
			};

			// Find the index of the pin to delete
			let pin_index_opt = node.pins.iter().position(|pin| {
				match (&pin.pin_type, &data.pin_type) {
					(PinType::PrePin(_), PinType::PrePin(_)) => true,
					(PinType::PostPin(_), PinType::PostPin(_)) => true,
					(PinType::MapperPin(_), PinType::MapperPin(_)) => true,
					(PinType::FilterPin(_), PinType::FilterPin(_)) => true,
					(PinType::LookupTransformPin(_), PinType::LookupTransformPin(_)) => true,
					(PinType::LookupFilterPin(_), PinType::LookupFilterPin(_)) => true,
					_ => false,
				}
			});

			let pin_index = match pin_index_opt {
				Some(idx) => idx,
				None => {
					return Err(ApiError::NotFound("NODE INDEX NOT FOUND".to_string()));
				}
			};

			// Delete the pin
			node.pins.remove(pin_index);
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
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

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
				// 1. Call `String::from_utf8()` on response.body
				// 2. We use a switch to explicitly call out both cases of decoding the Blob into ?Text
				let str_body = String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");

				return Ok(str_body);
			}
			Err((r, m)) => {
				let message = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

				//Return the error as a string and end the method
				return Err(ApiError::InterCanister(message));
			}
		}
	}

	/// Preview lookup HTTP Request
	///
	/// # Arguments
	/// - `data` - LookupHttpRequest
	///
	/// # Returns
	/// - `Unknown` - Unknown data from the canister
	pub async fn preview_http_request(
		data: LookupHttpRequest,
		_caller_principal: Principal
	) -> Result<String, ApiError> {
		// let canister_owner = CANISTER_OWNER.with(|canister_owner| canister_owner.borrow().get().clone());

		// Get HOST from URL
		let parsed_url = Url::parse(&data.url).expect("Failed to parse the URL");
		let host = match parsed_url.host_str() {
			Some(host) => host,
			None => {
				return Err(ApiError::NotFound("URL HOST NOT FOUND".to_string()));
			}
		};

		let mut request_headers = vec![
			HttpHeader {
				name: "Host".to_string(),
				value: format!("{host}:443"),
			},
			HttpHeader {
				name: "User-Agent".to_string(),
				value: "lookup_http_request".to_string(),
			}
		];

		let mut request_body: Option<Vec<u8>> = None;
		if data.method == HttpMethod::POST {
			// Add Idempotency-Key header if the method is POST
			request_headers.push(HttpHeader {
				name: "Idempotency-Key".to_string(),
				value: generate_idempotency_key().await.unwrap(),
			});

			// Prepare the request body
			if let Some(request_body_data) = data.request_body {
				let request_body_value: Value = serde_json::from_str(&request_body_data.to_string()).unwrap();
				request_body = Some(request_body_value.to_string().into_bytes());
			}
		}

		// Append the headers to "request_headers" from data.headers
		for header in data.headers {
			request_headers.push(HttpHeader {
				name: header.0,
				value: header.1,
			});
		}

		let request = CanisterHttpRequestArgument {
			url: data.url,
			method: data.method,
			body: request_body,
			max_response_bytes: None,
			transform: Some(TransformContext {
				// The "method" parameter needs to the same name as the function name of your transform function
				function: TransformFunc(candid::Func {
					principal: ic_cdk::api::id(),
					method: "transform".to_string(),
				}),
				// The "TransformContext" function does need a context parameter, it can be empty
				context: vec![],
			}),
			headers: request_headers,
		};

		match http_request(request, data.cycles).await {
			Ok((response,)) => {
				// if successful, `HttpResponse` has this structure:
				// pub struct HttpResponse {
				//     pub status: Nat,
				//     pub headers: Vec<HttpHeader>,
				//     pub body: Vec<u8>,
				// }

				// You need to decode that Vec<u8> that is the body into readable text.
				// To do this:
				//  1. Call `String::from_utf8()` on response.body
				//  2. You use a switch to explicitly call out both cases of decoding the Blob into ?Text

				// The API response will looks like this:

				// ("[[1682978460,5.714,5.718,5.714,5.714,243.5678]]")

				// Which can be formatted as this
				//  [
				//     [
				//         1682978460, <-- start/timestamp
				//         5.714, <-- low
				//         5.718, <-- high
				//         5.714, <-- open
				//         5.714, <-- close
				//         243.5678 <-- volume
				//     ],
				//  ]

				// Return the body as a string and end the method
				let str_body = String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");

				return Ok(str_body);
			}
			Err((r, m)) => {
				let message = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

				//Return the error as a string and end the method
				return Err(ApiError::InterCanister(message));
			}
		}
	}
}
