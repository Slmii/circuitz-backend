use candid::Principal;
use ic_cdk::api::time;
use crate::{
	lib::types::{ api_error::ApiError, circuit::{ Circuit, PostCircuit }, circuit_key::CircuitKey },
	storage::canister_storage::CIRCUITS,
};
pub struct CircuitsStore;

impl CircuitsStore {
	/// Get all circuits.
	///
	/// # Returns
	/// - `Vec<Circuit>` - Circuits
	pub fn get_circuits() -> Vec<Circuit> {
		CIRCUITS.with(|circuits| {
			let circuits = circuits.borrow();

			circuits
				.iter()
				.map(|(_, circuit)| circuit.clone())
				.collect::<Vec<Circuit>>()
		})
	}

	/// Get circuit by id.
	///
	/// # Arguments
	/// - `circuit_id` - Circuit ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Circuit` - Circuit
	pub fn get_circuit(circuit_id: u32, caller_principal: Principal) -> Result<Circuit, ApiError> {
		CIRCUITS.with(|circuits| {
			let circuits = circuits.borrow();

			// Find circuit by CircuitKey. If not found throw error
			let circuit_key = CircuitKey { id: circuit_id, owner: caller_principal.to_string() };
			let circuit = circuits.get(&circuit_key);

			if circuit.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			Ok(circuit.unwrap().clone())
		})
	}

	/// Get circuits by principal.
	///
	/// # Arguments
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Vec<Circuit>` - Circuits
	pub fn get_user_circuits(caller_principal: Principal) -> Vec<Circuit> {
		CIRCUITS.with(|circuits| {
			let circuits = circuits.borrow();

			circuits
				.iter()
				.filter(|(key, _)| caller_principal.to_string() == key.owner)
				.map(|(_, circuit)| circuit.clone())
				.collect::<Vec<Circuit>>()
		})
	}

	/// Get node canister ID.
	///
	/// # Arguments
	/// - `circuit_id` - Circuit ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Principal` - Canister ID of the node
	pub fn get_node_canister_id(circuit_id: u32, caller_principal: Principal) -> Result<Principal, ApiError> {
		let circuit = Self::get_circuit(circuit_id, caller_principal)?;

		Ok(circuit.node_canister_id)
	}

	/// Add circuit.
	///
	/// # Arguments
	/// - `caller_principal` - Principal of the caller
	/// - `data` - Circuit data
	///
	/// # Returns
	/// - `Circuit` - Added circuit
	pub fn add_circuit(data: PostCircuit, caller_principal: Principal) -> Circuit {
		CIRCUITS.with(|circuits| {
			let mut circuits = circuits.borrow_mut();

			let circuit_id =
				circuits
					.last_key_value()
					.map(|(key, _)| key.id)
					.unwrap_or(0) + 1;

			let new_circuit = Circuit {
				id: circuit_id,
				user_id: caller_principal,
				node_canister_id: Principal::anonymous(),
				name: data.name,
				description: data.description,
				is_favorite: false,
				is_running: false,
				is_enabled: false,
				run_at: None,
				created_at: time(),
				updated_at: time(),
			};

			// Add new circuit
			circuits.insert(CircuitKey { id: circuit_id, owner: caller_principal.to_string() }, new_circuit.clone());

			new_circuit
		})

		// TODO: add a new node canister, update "node_canister_id" in Circuit struct, make user the owner and add as a controller
	}

	/// Edit circuit.
	///
	/// # Arguments
	/// - `circuit_id` - Circuit ID
	/// - `data` - Circuit data
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Circuit` - Edited circuit
	pub fn edit_circuit(circuit_id: u32, data: PostCircuit, caller_principal: Principal) -> Result<Circuit, ApiError> {
		CIRCUITS.with(|circuits| {
			let mut circuits = circuits.borrow_mut();

			// Find mutable circuit by CircuitKey. If not found throw error
			let circuit_key = CircuitKey { id: circuit_id, owner: caller_principal.to_string() };
			let circuit = circuits.get(&circuit_key);

			if circuit.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			let mut circuit = circuit.unwrap().clone();

			// Mutate values
			circuit.name = data.name;
			circuit.description = data.description;
			circuit.updated_at = time();

			// Add new circuit or overwrite existing one
			circuits.insert(circuit_key, circuit.clone());

			Ok(circuit.clone())
		})
	}

	/// Toggle enable/disable circuit.
	///
	/// # Arguments
	/// - `circuit_id` - Circuit ID
	/// - `caller_principal` - Principal of the caller
	///
	/// # Returns
	/// - `Circuit` - Enabled circuit
	pub fn toggle_circuit(circuit_id: u32, enabled: bool, caller_principal: Principal) -> Result<Circuit, ApiError> {
		CIRCUITS.with(|circuits| {
			let mut circuits = circuits.borrow_mut();

			// Find mutable circuit by CircuitKey. If not found throw error
			let circuit_key = CircuitKey { id: circuit_id, owner: caller_principal.to_string() };
			let circuit = circuits.get(&circuit_key);

			if circuit.is_none() {
				return Err(ApiError::NotFound("NOT FOUND".to_string()));
			}

			let mut circuit = circuit.unwrap().clone();

			// Mutate values
			circuit.is_enabled = enabled;
			circuit.updated_at = time();

			// Add new circuit or overwrite existing one
			circuits.insert(circuit_key, circuit.clone());

			Ok(circuit.clone())
		})
	}
}
