use ic_stable_structures::{ memory_manager::{ MemoryManager, MemoryId }, DefaultMemoryImpl, StableBTreeMap };
use lib::types::node::Node;
use std::cell::RefCell;
use ic_stable_structures::memory_manager::VirtualMemory;

static NODES_MEMORY_ID: MemoryId = MemoryId::new(1);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

/// A reference to a `StableBTreeMap` that is wrapped in a `RefCell`.
///# Generics
/// * `K` - The key type of the `StableBTreeMap`.
/// * `V` - The value type of the `StableBTreeMap`.
pub type StorageRef<K, V> = RefCell<StableBTreeMap<K, V, Memory>>;
type MemManagerStore = RefCell<MemoryManager<DefaultMemoryImpl>>;

thread_local! {
	pub static MEMORY_MANAGER: MemManagerStore = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

	pub static NODES: StorageRef<u32, Node> = RefCell::new(
		StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(NODES_MEMORY_ID)))
	);
}
