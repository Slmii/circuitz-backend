use candid::Principal;
use ic_cdk::query;

pub mod canister_storage;

pub mod modules {
	pub mod nodes {
		pub mod nodes_controller;
		pub mod nodes_store;
	}
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
	use candid::export_service;
	use lib::types::api_error::*;
	use lib::types::node::*;
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
	write(dir.join(format!("nodes.did")), __export_did_tmp_()).expect("Write failed.");
}
