use candid::Principal;
use crate::{ types::api_error::ApiError, whitelist::whitelist };

/// Validate anonymous.
///
/// # Arguments
/// - `principal` - Principal
///
/// # Returns
/// - `Result<Principal, ApiError>` - Principal or ApiError
pub fn validate_anonymous(principal: &Principal) -> Result<Principal, ApiError> {
	Principal::from_text("2vxsx-fae").map_or(Err(ApiError::Unauthorized("UNAUTHORIZED".to_string())), |anon_principal| {
		if *principal == anon_principal {
			return Err(ApiError::Unauthorized("UNAUTHORIZED".to_string()));
		}

		return Ok(*principal);
	})
}

/// Validate admin.
///
/// # Arguments
/// - `principal` - Principal
///
/// # Returns
/// - `Result<Principal, ApiError>` - Principal or ApiError
pub fn validate_admin(principal: &Principal) -> Result<Principal, ApiError> {
	if !whitelist().contains(&principal) {
		return Err(ApiError::Unauthorized("UNAUTHORIZED".to_string()));
	}

	Ok(*principal)
}
