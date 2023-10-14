use candid::CandidType;
use serde::Deserialize;

use super::headers::Headers;

#[derive(CandidType, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HttpRequest {
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
