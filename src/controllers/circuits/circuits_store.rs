use candid::{ CandidType, Deserialize, Principal };
use ic_cdk::api::time;
use lib::types::circuit::{ Circuit, PostCircuit };
use std::{ cell::RefCell, collections::HashMap };

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct CircuitsStore {
	// Increment of circuit IDs
	pub circuit_id: u32,
	// All circuits in the system. u32 = circuit_id
	pub circuits: HashMap<u32, Circuit>,
	// Caller's circuits. Principal = caller, u32 = circuit_id
	pub user_circuits: HashMap<Principal, Vec<u32>>,
}

thread_local! {
	pub static STATE: RefCell<CircuitsStore> = RefCell::new(CircuitsStore::default());
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
		STATE.with(|state| {
			let state = state.borrow();

			// Get user's circuits
			let user_circuit_ids_by_principal = state.user_circuits.get(&caller_principal).cloned().unwrap_or_default();

			// Loop through all circuits and check if the circuit_id contains in user's circuits list
			state.circuits
				.values()
				.filter(|circuit| user_circuit_ids_by_principal.contains(&circuit.id))
				.cloned()
				.collect()
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
		STATE.with(|state| {
			let mut state = state.borrow_mut();

			// Increment circuit ID
			state.circuit_id += 1;
			let circuit_id = state.circuit_id;

			let new_circuit = Circuit {
				id: state.circuit_id,
				user_id: caller_principal,
				name: post_circuit.name,
				description: post_circuit.description,
				is_favorite: false,
				created_at: time(),
				updated_at: time(),
			};

			// Add new circuit
			state.circuits.insert(circuit_id, new_circuit.clone());

			// Add circuit to user_circuit
			state.user_circuits.entry(caller_principal).or_default().push(circuit_id);

			new_circuit
		})
	}
}
