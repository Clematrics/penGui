use std::any::TypeId;
use std::collections::HashMap;

pub type UniqueId = u64;

#[derive(Hash)]
pub struct CodeLocation(&'static str, u32, u32);

macro_rules! build {
    ( $expr:expr ) => {
        build(CodeLocation(file!(), line!(), column!()), expr)
    };
}

struct ComponentIdGenerator {
    // NOTE: A generator could just be a structure that holds associated functions
    _registry: HashMap<TypeId, UniqueId>,
}

impl ComponentIdGenerator {
    // NOTE: This might not be useful
    fn _new() -> Self {
        Self {
            _registry: HashMap::new(),
        }
    }

    fn generate_from_location<T: 'static>(loc: CodeLocation) -> ComponentId {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        loc.hash(&mut hasher);
        ComponentId::Generated(hasher.finish(), TypeId::of::<T>())
    }

    // NOTE: This might not be useful at all
    fn _generate<T: 'static>(&mut self) -> ComponentId {
        let type_id = TypeId::of::<T>();
        let counter = self._registry.entry(type_id).or_insert(0);
        *counter += 1;
        ComponentId::Generated(*counter, type_id)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum ComponentId {
    Generated(UniqueId, TypeId),
    Custom(UniqueId, TypeId),
}

impl ComponentId {
    pub fn new<T: 'static>(loc: CodeLocation) -> Self {
        ComponentIdGenerator::generate_from_location::<T>(loc)
    }

    pub fn new_custom<T: 'static>(id: UniqueId) -> Self {
        ComponentId::Custom(id, TypeId::of::<T>())
    }
}
