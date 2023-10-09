use candid::Principal;
use ic_cdk::api::time;
use ic_stable_structures::{
	memory_manager::{ VirtualMemory, MemoryManager, MemoryId },
	DefaultMemoryImpl,
	StableCell,
	StableBTreeMap,
};
use lib::types::circuit::{ Circuit, PostCircuit };
use std::cell::RefCell;
type Memory = VirtualMemory<DefaultMemoryImpl>;

pub struct CircuitsStore {}

thread_local! {
	pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
		MemoryManager::init(DefaultMemoryImpl::default())
	);

	// Circuit ID
	pub static CIRCUIT_ID: RefCell<StableCell<u32, Memory>> = RefCell::new(
		StableCell::init(
			MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
			0
		).expect("Failed to initialize CIRCUIT_ID")
	);

	// (circuit_id, principal) -> Circuit
	pub static CIRCUITS: RefCell<StableBTreeMap<(u32, String), Circuit, Memory>> = RefCell::new(
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
				.filter(|((_, principal), _)| caller_principal.to_string() == *principal)
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
		let circuit_id = CIRCUIT_ID.with(|circuit_id| {
			let mut circuit_id = circuit_id.borrow_mut().get().clone();

			// Increment circuit ID
			circuit_id += 1;

			circuit_id
		});

		CIRCUITS.with(|circuits| {
			let mut circuits = circuits.borrow_mut();

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
			circuits.insert((circuit_id, caller_principal.to_string()), new_circuit.clone());

			new_circuit
		})
	}
}
