use std::any::TypeId;
use std::collections::HashMap;

pub type UniqueId = u64;

#[derive(Hash, Copy, Clone)]
pub struct CodeLocation(pub &'static str, pub u32, pub u32);

pub fn new_code_location(s: &'static str, l: u32, r: u32) -> CodeLocation {
    CodeLocation(s, l, r)
}

#[macro_export]
macro_rules! build {
    ( $root:expr, $expr:expr ) => {
        root.build(CodeLocation(file!(), line!(), column!()), expr)
    };
}

#[macro_export]
macro_rules! loc {
    () => {
        CodeLocation(file!(), line!(), column!())
        // CodeLocation{0: file!(), 1: line!(), 2: column!()}
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
