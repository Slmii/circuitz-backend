use candid::Principal;
use crate::lib::types::api_error::ApiError;

pub async fn generate_idempotency_key() -> Result<String, ApiError> {
	let result: Result<(Vec<u8>,), _> = ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ()).await;

	match result {
		Ok((bytes,)) => {
			let random_string = format!(
				"UUID-{}",
				bytes
					.iter()
					.map(|b| format!("{:02x}", b))
					.collect::<String>()
			);

			Ok(random_string)
		}
		Err(_) => Err(ApiError::InterCanister("Failed to generate random number".to_string())),
	}
}
