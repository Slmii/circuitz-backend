use candid::Principal;
use ic_cdk::query;

pub mod canister_storage;

pub mod modules {
	pub mod circuits {
		pub mod circuits_controller;
		pub mod circuits_store;
	}

	pub mod connectors {
		pub mod connectors_controller;
		pub mod connectors_store;
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
	use lib::types::trace::*;
	use lib::types::user::*;
	use lib::types::connector::*;

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
