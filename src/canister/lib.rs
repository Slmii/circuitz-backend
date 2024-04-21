use candid::Principal;
use ic_cdk::query;

pub mod lib {
	pub mod types {
		pub mod api_error;
		pub mod circuit;
		pub mod circuit_key;
		pub mod node;
		pub mod user;
		pub mod trace;
		pub mod trace_key;
		pub mod headers;
	}

	pub mod utils {
		pub mod idempotency;
		pub mod validate;
	}

	pub mod macros;
	pub mod whitelist;
}

pub mod storage {
	pub mod canister_storage;
}

pub mod modules {
	pub mod circuits {
		pub mod circuits_controller;
		pub mod circuits_store;
	}

	pub mod nodes {
		pub mod nodes_controller;
		pub mod nodes_store;
	}

	pub mod traces {
		pub mod traces_controller;
		pub mod traces_store;
	}

	pub mod users {
		pub mod users_controller;
		pub mod users_store;
	}
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
	use candid::export_service;
	use lib::types::api_error::*;
	use lib::types::circuit::*;
	use lib::types::node::*;
	use lib::types::trace::*;
	use lib::types::user::*;
	use ic_cdk::api::management_canister::http_request::{ TransformArgs, HttpResponse };

	export_service!();
	__export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
	use std::env;
	use std::fs::write;
	use std::path::PathBuf;

	let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let dir = dir.parent().unwrap().parent().unwrap().join("candid");
	write(dir.join(format!("canister.did")), __export_did_tmp_()).expect("Write failed.");
}
