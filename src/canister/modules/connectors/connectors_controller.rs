use ic_cdk::{ caller, query };
use lib::{ types::{ api_error::ApiError, connector::Connector }, utils::validate::validate_anonymous };
use super::connectors_store::ConnectorsStore;

#[query]
fn get_user_connectors() -> Result<Vec<Connector>, ApiError> {
	match validate_anonymous(&caller()) {
		Ok(caller_principal) => Ok(ConnectorsStore::get_user_connectors(caller_principal)),
		Err(err) => Err(err),
	}
}
