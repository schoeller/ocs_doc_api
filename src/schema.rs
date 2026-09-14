use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Stable identifier for a type in the generated object model.
pub type TypeId = String;

/// The structural kind of a type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeKind {
    /// A primitive scalar such as `f64`, `i32`, `String`, or `bool`.
    Primitive,
    /// A struct with named fields.
    Struct,
    /// An enum with named variants.
    Enum,
    /// A tuple struct with unnamed fields.
    TupleStruct,
    /// A newtype struct wrapping another type.
    NewTypeStruct,
    /// A unit struct with no data.
    UnitStruct,
    /// A sequence such as `Vec<T>`.
    Seq,
    /// An optional value.
    Option,
    /// A map such as `HashMap<K, V>`.
    Map,
    /// A tuple with unnamed fields.
    Tuple,
    /// A fixed-size array such as `[T; N]`.
    Array,
}

/// Metadata for one field of a struct or enum variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldInfo {
    pub name: String,
    pub type_id: TypeId,
    /// Whether the field is required in an entity payload.
    /// Skipped or defaulted fields are not required.
    pub required: bool,
}

/// Metadata for one enum variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumVariantInfo {
    pub name: String,
    /// Index of this variant used by `serde-reflection`.
    pub index: u32,
    /// For tuple/struct variants, the fields in order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<FieldInfo>,
}

/// A kernel capability attached to an entity type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    /// Human-readable return type of the kernel operation.
    pub output: String,
    /// Optional constraints, e.g. "planar, non-self-intersecting".
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub requires: String,
}

/// Full metadata for one type in the object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeInfo {
    pub kind: TypeKind,
    /// Stable name used in payloads and dispatch.
    pub name: String,
    /// Underlying Rust type path, when known.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub rust_type: String,
    /// Fields for structs and struct-like enum variants.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<FieldInfo>,
    /// Variants for enums.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<EnumVariantInfo>,
    /// Inner type id for newtype structs, sequences, options, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner: Option<TypeId>,
    /// Fixed array length.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub len: Option<usize>,
    /// Key and value types for maps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<TypeId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<TypeId>,
    /// Kernel capabilities discovered for this entity type.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
}

/// The complete generated object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeRegistry {
    /// Map from stable type name to its metadata.
    pub types: BTreeMap<TypeId, TypeInfo>,
}

#[allow(dead_code)]
impl TypeRegistry {
    /// Look up a type by its stable name.
    pub fn get(&self, name: &str) -> Option<&TypeInfo> {
        self.types.get(name)
    }

    /// Return the capabilities attached to a type, if any.
    pub fn capabilities_for(&self, name: &str) -> &[Capability] {
        self.get(name)
            .map(|info| info.capabilities.as_slice())
            .unwrap_or(&[])
    }

    /// Return all type names in the registry.
    pub fn type_names(&self) -> Vec<&str> {
        self.types.keys().map(|s| s.as_str()).collect()
    }
}
