use candid::types::principal::Principal;
use ic_cdk::{ caller, api::management_canister::http_request::{ TransformArgs, HttpResponse, HttpHeader } };
use ic_cdk_macros::{ query, update };
use lib::{
	types::{ node::{ Node, Pin, NodeType, LookupCanister }, api_error::ApiError },
	utils::validate::validate_anonymous,
};
use crate::nodes_store::NodesStore;

#[query]
fn get_circuit_node(node_id: u32) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::get_circuit_node(node_id, caller_principal),
		Err(err) => Err(err),
	}
}

#[query]
fn get_circuit_nodes(circuit_id: u32) -> Result<(Principal, Vec<Node>), ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::get_circuit_nodes(circuit_id, caller_principal),
		Err(err) => Err(err),
	}
}

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
	let headers = vec![
		HttpHeader {
			name: "Content-Security-Policy".to_string(),
			value: "default-src 'self'".to_string(),
		},
		HttpHeader {
			name: "Referrer-Policy".to_string(),
			value: "strict-origin".to_string(),
		},
		HttpHeader {
			name: "Permissions-Policy".to_string(),
			value: "geolocation=(self)".to_string(),
		},
		HttpHeader {
			name: "Strict-Transport-Security".to_string(),
			value: "max-age=63072000".to_string(),
		},
		HttpHeader {
			name: "X-Frame-Options".to_string(),
			value: "DENY".to_string(),
		},
		HttpHeader {
			name: "X-Content-Type-Options".to_string(),
			value: "nosniff".to_string(),
		}
	];

	let mut res = HttpResponse {
		status: raw.response.status.clone(),
		body: raw.response.body.clone(),
		headers,
		..Default::default()
	};

	if res.status == 200 {
		res.body = raw.response.body;
	} else {
		ic_cdk::api::print(format!("Received an error: err = {:?}", raw));
	}
	res
}

#[update]
fn delete_node(node_id: u32) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::delete_node(node_id, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn add_node(circuit_id: u32, data: NodeType) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::add_node(circuit_id, data, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn edit_node(node_id: u32, data: NodeType) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::edit_node(node_id, data, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn edit_order(node_id: u32, order: u32) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::edit_order(node_id, order, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn add_pin(node_id: u32, data: Pin) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::add_pin(node_id, data, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn edit_pin(node_id: u32, data: Pin) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::edit_pin(node_id, data, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn delete_pin(node_id: u32, data: Pin) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::delete_pin(node_id, data, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn enable_node(node_id: u32) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::toggle_node(node_id, true, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
fn disable_node(node_id: u32) -> Result<Node, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::toggle_node(node_id, false, caller_principal),
		Err(err) => Err(err),
	}
}

#[update]
async fn preview_lookup_request(data: LookupCanister) -> Result<String, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => NodesStore::preview_lookup_request(data, caller_principal).await,
		Err(err) => Err(err),
	}
}

// #[query]
// fn test(json: String, key: String) -> String {
// 	let v: Value = serde_json::from_str(json.as_str()).expect("JSON was not well-formatted");

// 	match v {
// 		Value::Object(_) => {}
// 		Value::Array(array) => {}
// 		Value::Bool(_) => {}
// 		Value::Null => {}
// 		Value::Number(_) => {}
// 		Value::String(_) => {}
// 	}

// 	let name = v[key].as_str().unwrap();

// 	name.to_string()
// }

// #[init]
// #[candid_method(init)]
// fn init(canister_owner: Option<Principal>) {
// 	// TODO: add controller and owner to created caniser
// }

#[test]
fn generate_candid() {
	use candid::export_service;
	use lib::save_candid;
	export_service!();

	save_candid::save_candid(__export_service(), "nodes".to_string());
}
