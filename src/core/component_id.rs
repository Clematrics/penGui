use std::any::TypeId;

/// Type to describe unique identifiers of components
pub type UniqueId = u64;

/// Structure containing a location of some code with
/// - a file name
/// - the line number
/// - the column number
#[derive(Hash, Copy, Clone)]
pub struct CodeLocation(pub &'static str, pub u32, pub u32);

/// Postfix macro to quickly build a widget into an interface
///
/// NOTE: This macro is not usable as long as postfix macros are not supported by Rust
/// See [#2442](https://github.com/rust-lang/rfcs/pull/2442) for more details on the current
/// status of postfix macros in Rust.
#[macro_export]
macro_rules! build {
    ( $root:expr, $expr:expr ) => {
        $root.expr(CodeLocation(file!(), line!(), column!()), $expr.clone())
    };
}

/// Macro constructing a CodeLocation, based on the current file, the line and column it is called at
#[macro_export]
macro_rules! loc {
    () => {
        CodeLocation(file!(), line!(), column!())
    };
}

/// An identifier for any widget, based on a unique number (a hash being the more suitable)
/// and on its type
/// It can be either a generated one or a custom one
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum ComponentId {
    Generated(UniqueId, TypeId),
    Custom(UniqueId, TypeId),
}

impl ComponentId {
    /// Creates a new `ComponentId` from a `CodeLocation`
    pub fn new<T: 'static>(loc: CodeLocation) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        loc.hash(&mut hasher);
        ComponentId::Generated(hasher.finish(), TypeId::of::<T>())
    }

    /// Creates a new `ComponentId` from a `CodeLocation` and an additionnal number
    pub fn new_biased<T: 'static>(loc: CodeLocation, id: UniqueId) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        loc.hash(&mut hasher);
        id.hash(&mut hasher);
        ComponentId::Generated(hasher.finish(), TypeId::of::<T>())
    }

    /// Creates a new, fully custom `ComponentId` from the provided number
    pub fn new_custom<T: 'static>(id: UniqueId) -> Self {
        ComponentId::Custom(id, TypeId::of::<T>())
    }
}
