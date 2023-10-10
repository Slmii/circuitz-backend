use candid::Principal;
use ic_cdk::api::time;
use ic_stable_structures::{ memory_manager::{ MemoryManager, MemoryId }, DefaultMemoryImpl, StableBTreeMap };
use lib::types::{ circuit::{ Circuit, PostCircuit, CircuitKey }, memory::Memory };
use std::cell::RefCell;

pub struct CircuitsStore {}

thread_local! {
	pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
		MemoryManager::init(DefaultMemoryImpl::default())
	);

	pub static CIRCUITS: RefCell<StableBTreeMap<CircuitKey, Circuit, Memory>> = RefCell::new(
		StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
	);
}

impl CircuitsStore {
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

	/// Add circuit.
	///
	/// # Arguments
	/// - `caller_principal` - Principal of the caller
	/// - `post_circuit` - Circuit to add
	///
	/// # Returns
	/// - `Circuit` - Added circuit
	pub fn add_circuit(post_circuit: PostCircuit, caller_principal: Principal) -> Circuit {
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
				name: post_circuit.name,
				description: post_circuit.description,
				is_favorite: false,
				is_enabled: false,
				run_at: None,
				created_at: time(),
				updated_at: time(),
			};

			// Add new circuit
			circuits.insert(CircuitKey { id: circuit_id, owner: caller_principal.to_string() }, new_circuit.clone());

			new_circuit
		})
	}
}
