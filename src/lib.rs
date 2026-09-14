//! `ocs_doc_api` — stable object model and transport-agnostic document API for
//! the OpenCADStudio drawing kernel.

pub mod schema;

#[cfg(any(test, feature = "engine", feature = "kernel"))]
pub mod entity_samples;

#[cfg(feature = "engine")]
pub mod doc_api;

#[cfg(feature = "kernel")]
pub mod convert;

#[cfg(feature = "kernel")]
pub mod kernel_ops;

#[cfg(feature = "engine")]
pub mod in_process;

pub use schema::{Capability, TypeInfo, TypeRegistry};
use serde::{Deserialize, Serialize};

#[cfg(feature = "engine")]
pub use doc_api::{
    entity_type_name, validate_entity_payload, DocApi, DocApiError, DocApiExt, DocOp,
    DocumentSnapshot, EntityPayload, Receipt,
};

/// Public 64-bit handle wrapper with conversions to/from `acadrust::Handle`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Handle(pub u64);

impl From<u64> for Handle {
    fn from(value: u64) -> Self {
        Handle(value)
    }
}

impl From<Handle> for u64 {
    fn from(handle: Handle) -> Self {
        handle.0
    }
}

/// The embedded object model as a JSON string.
pub const EMBEDDED_OBJECT_MODEL_JSON: &str =
    include_str!(concat!(env!("OUT_DIR"), "/object_model.json"));

/// Markdown documentation for the object model.
pub const EMBEDDED_OBJECT_MODEL_DOCS_MD: &str =
    include_str!(concat!(env!("OUT_DIR"), "/object_model_docs.md"));

/// Markdown documentation for the `DocApi` operations.
pub const EMBEDDED_DOC_API_OPS_MD: &str =
    include_str!(concat!(env!("OUT_DIR"), "/doc_api_ops.md"));

#[cfg(feature = "kernel")]
include!(concat!(env!("OUT_DIR"), "/object_model_dispatch.rs"));

/// Parse the embedded object model into a `TypeRegistry`.
pub fn object_model() -> TypeRegistry {
    serde_json::from_str(EMBEDDED_OBJECT_MODEL_JSON)
        .expect("embedded object model is valid JSON")
}

/// Return the capabilities attached to a type by its stable name.
pub fn capabilities_for(name: &str) -> Vec<Capability> {
    object_model()
        .get(name)
        .map(|info| info.capabilities.clone())
        .unwrap_or_default()
}

/// Return the embedded object-model Markdown documentation.
pub fn embedded_object_model_docs_md() -> &'static str {
    EMBEDDED_OBJECT_MODEL_DOCS_MD
}

/// Return the embedded DocApi operations Markdown documentation.
pub fn embedded_doc_api_ops_md() -> &'static str {
    EMBEDDED_DOC_API_OPS_MD
}
