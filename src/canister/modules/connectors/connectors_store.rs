use candid::Principal;
use lib::types::connector::Connector;
use crate::canister_storage::CONNECTORS;

pub struct ConnectorsStore;

impl ConnectorsStore {
	/// Get connectors by principal.
	///
	/// # Arguments
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Vec<Connector>` - Connectors
	pub fn get_user_connectors(caller_principal: Principal) -> Vec<Connector> {
		CONNECTORS.with(|connectors| {
			let connecetors = connectors.borrow();

			connecetors
				.iter()
				.filter(|(key, _)| caller_principal.to_string() == key.owner)
				.map(|(_, circuit)| circuit.clone())
				.collect::<Vec<Connector>>()
		})
	}
}
