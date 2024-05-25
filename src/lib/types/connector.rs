use candid::{ CandidType, Deserialize, Principal };
use crate::impl_storable_for;
use super::{ headers::Headers, node::HttpRequestMethod };

impl_storable_for!(Connector);
#[derive(CandidType, Clone, Deserialize)]
pub struct Connector {
	pub id: u32,
	pub user_id: Principal,
	pub name: String,
	pub connector_type: ConnectorType,
	pub created_at: u64,
	pub updated_at: u64,
}

impl Default for Connector {
	fn default() -> Self {
		Self {
			id: Default::default(),
			user_id: Principal::anonymous(),
			name: Default::default(),
			connector_type: ConnectorType::Http(HttpConnector {
				base_url: Default::default(),
				headers: Default::default(),
				authentication: Authentication::None,
				test_connection: Default::default(),
			}),
			created_at: Default::default(),
			updated_at: Default::default(),
		}
	}
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PostConnector {
	pub name: String,
	pub connector_type: ConnectorType,
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum ConnectorType {
	Http(HttpConnector),
	Canister(String),
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct HttpConnector {
	pub base_url: String,
	pub headers: Headers,
	pub authentication: Authentication,
	pub test_connection: Option<TestConnection>,
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum Authentication {
	None,
	Basic(String, String), // username, password
	JWT(JWTConfig),
	Token(TokenConfig),
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct JWTConfig {
	pub signature_method: SignatureMethod,
	pub secret: String,
	pub secret_key: String,
	pub payload: String,
	pub location: TokenLocation,
	pub sample_data: String,
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum SignatureMethod {
	HS256,
	HS384,
	HS512,
	RS256,
	RS384,
	RS512,
	ES256,
	ES384,
	ES512,
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TokenConfig {
	pub token: String,
	pub location: TokenLocation,
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum TokenLocation {
	HTTPHeader((String, String)), // (header_name, header_scheme)
	Query(String),
}

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TestConnection {
	pub relative_url: String,
	pub method: HttpRequestMethod,
	pub error: Option<(String, String)>, // (error_field, error_value)
}
