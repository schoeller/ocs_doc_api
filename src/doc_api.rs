use crate::schema::{FieldInfo, TypeRegistry};
use crate::Handle;
use serde::{Deserialize, Serialize};
/// A serializable entity representation used by `DocApi` operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityPayload {
    /// Canonical entity kind, e.g. `line`, `circle`, `lwpolyline`.
    pub kind: String,
    /// The entity-specific fields as a JSON object.
    pub data: serde_json::Value,
}

impl EntityPayload {
    /// Create a payload from a typed value.
    ///
    /// `kind` must be a known entity kind and `value` must serialize to the
    /// expected entity structure.
    pub fn new(kind: impl Into<String>, data: serde_json::Value) -> Self {
        Self {
            kind: kind.into(),
            data,
        }
    }
}

/// Convert a typed `EntityType` into a serializable payload.
#[cfg(feature = "engine")]
pub fn entity_to_payload(entity: &acadrust::entities::EntityType) -> Result<EntityPayload, DocApiError> {
    let value = serde_json::to_value(entity)?;
    let object = value.as_object().ok_or_else(|| DocApiError::Serialize {
        message: "entity did not serialize to a JSON object".into(),
    })?;
    let (kind, data) = object
        .iter()
        .next()
        .ok_or_else(|| DocApiError::Serialize {
            message: "entity serialized to an empty object".into(),
        })?;
    Ok(EntityPayload {
        kind: kind.to_lowercase(),
        data: data.clone(),
    })
}

/// Lightweight snapshot of a document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentSnapshot {
    pub name: String,
    pub entities: Vec<EntityPayload>,
}

/// Boolean/set operations available for future kernel work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoolOp {
    Union,
    Difference,
    Intersection,
}

/// A single document operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum DocOp {
    CreateDocument { name: String },
    GetDocument,
    ListEntities,
    CreateEntity { payload: EntityPayload },
    ReadEntity { handle: Handle },
    UpdateEntity {
        handle: Handle,
        payload: EntityPayload,
    },
    DeleteEntity { handle: Handle },
    OffsetEntity {
        handle: Handle,
        distance: f64,
        /// Optional pick side for the offset; defaults to positive offset.
        side: Option<[f64; 2]>,
    },
    /// Placeholder for operations not implemented in the first milestone.
    #[serde(other)]
    Unsupported,
}

/// Result of a single `DocOp`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum Receipt {
    DocumentCreated { handle: Handle },
    DocumentSnapshot(DocumentSnapshot),
    EntityList { entities: Vec<(Handle, EntityPayload)> },
    EntityCreated { handle: Handle },
    EntityRead { handle: Handle, payload: EntityPayload },
    EntityUpdated { handle: Handle },
    EntityDeleted { handle: Handle },
    OffsetResult { source: Handle, results: Vec<EntityPayload> },
    Error { message: String },
}

/// Errors returned by `DocApi` implementations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum DocApiError {
    NotImplemented { op: String },
    EntityNotFound { handle: Handle },
    InvalidPayload { message: String },
    KernelOpFailed { op: String, message: String },
    Serialize { message: String },
    Deserialize { message: String },
    Io { message: String },
}

impl std::fmt::Display for DocApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DocApiError {}

impl From<serde_json::Error> for DocApiError {
    fn from(err: serde_json::Error) -> Self {
        DocApiError::Serialize {
            message: err.to_string(),
        }
    }
}

/// Transport-agnostic document API.
pub trait DocApi {
    /// Execute one operation and return its receipt.
    fn execute(&mut self, op: DocOp) -> Result<Receipt, DocApiError>;
}

/// Convenience methods on top of `DocApi`.
pub trait DocApiExt: DocApi {
    fn create_document(&mut self, name: impl Into<String>) -> Result<Receipt, DocApiError> {
        self.execute(DocOp::CreateDocument { name: name.into() })
    }

    fn get_document(&mut self) -> Result<Receipt, DocApiError> {
        self.execute(DocOp::GetDocument)
    }

    fn list_entities(&mut self) -> Result<Receipt, DocApiError> {
        self.execute(DocOp::ListEntities)
    }

    fn create_entity(&mut self, payload: EntityPayload) -> Result<Receipt, DocApiError> {
        self.execute(DocOp::CreateEntity { payload })
    }

    fn read_entity(&mut self, handle: Handle) -> Result<Receipt, DocApiError> {
        self.execute(DocOp::ReadEntity { handle })
    }

    fn update_entity(
        &mut self,
        handle: Handle,
        payload: EntityPayload,
    ) -> Result<Receipt, DocApiError> {
        self.execute(DocOp::UpdateEntity { handle, payload })
    }

    fn delete_entity(&mut self, handle: Handle) -> Result<Receipt, DocApiError> {
        self.execute(DocOp::DeleteEntity { handle })
    }

    fn offset_entity(
        &mut self,
        handle: Handle,
        distance: f64,
        side: Option<[f64; 2]>,
    ) -> Result<Receipt, DocApiError> {
        self.execute(DocOp::OffsetEntity {
            handle,
            distance,
            side,
        })
    }
}

impl<T: DocApi + ?Sized> DocApiExt for T {}

/// Map a canonical entity kind to the registry type name.
pub fn entity_type_name(kind: &str) -> &str {
    match kind {
        "line" => "Line",
        "circle" => "Circle",
        "lwpolyline" => "LwPolyline",
        "arc" => "Arc",
        "ellipse" => "Ellipse",
        "point" => "Point",
        "text" => "Text",
        "mtext" => "MText",
        "spline" => "Spline",
        "polyline" => "Polyline",
        "polyline2d" => "Polyline2D",
        "polyline3d" => "Polyline3D",
        "insert" => "Insert",
        "hatch" => "Hatch",
        _ => kind, // fallback for unknown / already-cased names
    }
}

/// Validate an `EntityPayload` against the generated registry.
///
/// Checks:
/// - `kind` maps to a known struct type.
/// - `data` is a JSON object.
/// - Every required field declared by the registry is present.
///
/// It does not type-check individual field values beyond JSON shape.
pub fn validate_entity_payload(payload: &EntityPayload, registry: &TypeRegistry) -> Result<(), DocApiError> {
    let type_name = entity_type_name(&payload.kind);
    let info = registry
        .get(type_name)
        .ok_or_else(|| DocApiError::InvalidPayload {
            message: format!("unknown entity kind: {}", payload.kind),
        })?;

    if info.kind != crate::schema::TypeKind::Struct {
        return Err(DocApiError::InvalidPayload {
            message: format!("{} is not a struct entity type", type_name),
        });
    }

    let object = payload
        .data
        .as_object()
        .ok_or_else(|| DocApiError::InvalidPayload {
            message: "entity data must be a JSON object".into(),
        })?;

    let required: Vec<&FieldInfo> = info.fields.iter().filter(|f| f.required).collect();
    for field in required {
        if !object.contains_key(&field.name) {
            return Err(DocApiError::InvalidPayload {
                message: format!(
                    "missing required field '{}' on {}",
                    field.name, type_name
                ),
            });
        }
    }

    Ok(())
}

/// Validate a list of operation metadata entries against the `DocOp` enum.
///
/// Used by tests to keep `doc_api_ops.toml` in sync with the Rust enum.
pub fn validate_doc_op_names(names: &[String]) -> Result<(), String> {
    const VARIANTS: &[&str] = &[
        "CreateDocument",
        "GetDocument",
        "ListEntities",
        "CreateEntity",
        "ReadEntity",
        "UpdateEntity",
        "DeleteEntity",
        "OffsetEntity",
        "Unsupported",
    ];
    let known: std::collections::HashSet<&str> = VARIANTS.iter().copied().collect();
    for name in names {
        if !known.contains(name.as_str()) {
            return Err(format!(
                "doc_api_ops.toml contains unknown DocOp variant: {}",
                name
            ));
        }
    }
    Ok(())
}
