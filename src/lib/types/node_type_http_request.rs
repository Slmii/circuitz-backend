use candid::CandidType;
use serde::Deserialize;

use super::headers::Headers;

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HttpRequest {
	name: String,
	description: Option<String>,
	url: String,
	method: HttpRequestMethod,
	// Store header name and value
	headers: Headers,
	request_body: Option<String>,
}

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum HttpRequestMethod {
	GET,
	POST,
}
