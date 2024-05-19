use ic_cdk::{ caller, query, update };
use lib::{
	types::{ api_error::ApiError, connector::{ Connector, PostConnector } },
	utils::validate::validate_anonymous,
};
use super::connectors_store::ConnectorsStore;

#[query]
fn get_user_connectors() -> Result<Vec<Connector>, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(ConnectorsStore::get_user_connectors(caller_principal)),
		Err(err) => Err(err),
	}
}

#[update]
fn add_connector(data: PostConnector) -> Result<Connector, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(ConnectorsStore::add_connector(data, caller_principal)),
		Err(err) => Err(err),
	}
}

#[update]
fn edit_connector(connector_id: u32, data: PostConnector) -> Result<Connector, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => ConnectorsStore::edit_connector(connector_id, data, caller_principal),
		Err(err) => Err(err),
	}
}
