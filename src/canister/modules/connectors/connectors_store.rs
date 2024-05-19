use candid::Principal;
use ic_cdk::api::time;
use lib::types::{ api_error::ApiError, connector::{ Connector, PostConnector }, connector_key::ConnectorKey };
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
				.map(|(_, connector)| connector.clone())
				.collect::<Vec<Connector>>()
		})
	}

	/// Add connector.
	///
	/// # Arguments
	/// - `data` - Connector data
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Connector` - Added connector
	pub fn add_connector(data: PostConnector, caller_principal: Principal) -> Connector {
		CONNECTORS.with(|connectors| {
			let mut connectors = connectors.borrow_mut();

			let connector_id =
				connectors
					.last_key_value()
					.map(|(key, _)| key.id)
					.unwrap_or(0) + 1;

			let new_connector = Connector {
				id: connector_id,
				user_id: caller_principal,
				name: data.name,
				connector_type: data.connector_type,
				created_at: time(),
				updated_at: time(),
			};

			// Add new connector
			connectors.insert(
				ConnectorKey { id: connector_id, owner: caller_principal.to_string() },
				new_connector.clone()
			);

			new_connector
		})
	}

	/// Edit connector.
	///
	/// # Arguments
	/// - `connector_id` - Connector ID
	/// - `data` - Connector data
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Connector` - Edited connector
	pub fn edit_connector(
		connector_id: u32,
		data: PostConnector,
		caller_principal: Principal
	) -> Result<Connector, ApiError> {
		CONNECTORS.with(|connectors| {
			let mut connectors = connectors.borrow_mut();

			// Find mutable connector by ConnectorKey. If not found throw error
			let connector_key = ConnectorKey { id: connector_id, owner: caller_principal.to_string() };
			let connector = connectors.get(&connector_key);

			if connector.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			let mut connector = connector.unwrap().clone();

			// Mutate values
			connector.name = data.name;
			connector.connector_type = data.connector_type;
			connector.updated_at = time();

			// Add new connector or overwrite existing one
			connectors.insert(connector_key, connector.clone());

			Ok(connector.clone())
		})
	}
}
