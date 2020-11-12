use std::any::TypeId;
use std::collections::HashMap;

pub type UniqueId = u64;

struct ComponentIdGenerator {
	registry: HashMap<TypeId, UniqueId>
}

static component_id_generator: ComponentIdGenerator = ComponentIdGenerator {
	registry: HashMap::new()
};

impl ComponentIdGenerator {
	fn generate<T: 'static>() -> UniqueId {
		let counter = component_id_generator.registry.entry(TypeId::of::<T>()).or_insert(0);
		*counter += 1;
		*counter
	}
}

#[derive(PartialEq, Eq, Hash)]
pub enum ComponentId {
	Generated(UniqueId, TypeId),
	Custom(UniqueId, TypeId)
}

impl ComponentId {
	pub fn new<T: 'static>() -> Self {
		ComponentId::Generated(ComponentIdGenerator::generate::<T>(), TypeId::of::<T>())
	}

	pub fn new_custom<T: 'static>(id: UniqueId) -> Self {
		ComponentId::Custom(id, TypeId::of::<T>())
	}
}