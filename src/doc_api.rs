use crate::schema::{FieldInfo, TypeRegistry};
use crate::Handle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
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

const ENTITY_TYPE_NAMES: &[(&str, &str)] = &[
    ("arc", "Arc"),
    ("attributedefinition", "AttributeDefinition"),
    ("attributeentity", "AttributeEntity"),
    ("block", "Block"),
    ("blockend", "BlockEnd"),
    ("body", "Body"),
    ("circle", "Circle"),
    ("dimension", "Dimension"),
    ("ellipse", "Ellipse"),
    ("extended", "Extended"),
    ("face3d", "Face3D"),
    ("hatch", "Hatch"),
    ("helix", "Helix"),
    ("insert", "Insert"),
    ("leader", "Leader"),
    ("light", "Light"),
    ("line", "Line"),
    ("lwpolyline", "LwPolyline"),
    ("mesh", "Mesh"),
    ("mline", "MLine"),
    ("mtext", "MText"),
    ("multileader", "MultiLeader"),
    ("ole2frame", "Ole2Frame"),
    ("point", "Point"),
    ("polyface_mesh", "PolyfaceMesh"),
    ("polyfacemesh", "PolyfaceMesh"),
    ("polygon_mesh", "PolygonMesh"),
    ("polygonmesh", "PolygonMesh"),
    ("polyline", "Polyline"),
    ("polyline2d", "Polyline2D"),
    ("polyline3d", "Polyline3D"),
    ("raster_image", "RasterImage"),
    ("rasterimage", "RasterImage"),
    ("ray", "Ray"),
    ("region", "Region"),
    ("section_symbol", "SectionSymbol"),
    ("sectionsymbol", "SectionSymbol"),
    ("seqend", "Seqend"),
    ("shape", "Shape"),
    ("solid", "Solid"),
    ("solid3d", "Solid3D"),
    ("spline", "Spline"),
    ("surface", "Surface"),
    ("table", "Table"),
    ("text", "Text"),
    ("tolerance", "Tolerance"),
    ("underlay", "Underlay"),
    ("unknown", "Unknown"),
    ("view_border", "ViewBorder"),
    ("viewborder", "ViewBorder"),
    ("viewport", "Viewport"),
    ("wipeout", "Wipeout"),
    ("xline", "XLine"),
];

fn entity_type_name_map() -> &'static HashMap<&'static str, &'static str> {
    static MAP: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    MAP.get_or_init(|| ENTITY_TYPE_NAMES.iter().copied().collect())
}

/// Map a canonical entity kind to the registry type name.
pub fn entity_type_name(kind: &str) -> &str {
    entity_type_name_map()
        .get(kind)
        .copied()
        .unwrap_or(kind)
}

/// Validate an `EntityPayload` against the generated registry.
///
/// Checks:
/// - `kind` maps to a known struct type.
/// - `data` is a JSON object.
/// - Every required field declared by the registry is present.
/// - No unknown fields are present.
/// - Each supplied field's JSON shape matches the declared type.
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

    let field_map: std::collections::HashMap<&str, &FieldInfo> =
        info.fields.iter().map(|f| (f.name.as_str(), f)).collect();
    for (key, value) in object {
        let Some(field) = field_map.get(key.as_str()) else {
            return Err(DocApiError::InvalidPayload {
                message: format!(
                    "unknown field '{}' on {}",
                    key, type_name
                ),
            });
        };
        // Optional fields may be supplied as `null` even when the sample
        // used to derive the registry contained a value.
        if value.is_null() && !field.required {
            continue;
        }
        if !json_value_matches_type_id(value, &field.type_id) {
            return Err(DocApiError::InvalidPayload {
                message: format!(
                    "field '{}' on {} does not match type {}",
                    key, type_name, field.type_id
                ),
            });
        }
    }

    Ok(())
}

/// Check whether a JSON value's shape matches a registry type id.
///
/// This is intentionally shallow: it catches obvious category mismatches
/// (object vs array vs scalar) without requiring full nested schema lookups.
fn json_value_matches_type_id(value: &serde_json::Value, type_id: &str) -> bool {
    // Strip whitespace from generic arguments (e.g. "Map< String, EntityCommon >")
    // without allocating a new String.
    fn trim_id(s: &str) -> &str {
        s.trim_matches(|c: char| c.is_whitespace())
    }

    let type_id = trim_id(type_id);
    if type_id == "unknown" {
        // The registry has no concrete type information for this field, so any
        // supplied value is accepted rather than rejected because of a coarse
        // fallback entry.
        return true;
    }
    if type_id.starts_with("Option<") && type_id.ends_with('>') {
        let inner = trim_id(&type_id["Option<".len()..type_id.len() - 1]);
        return value.is_null() || json_value_matches_type_id(value, inner);
    }
    match value {
        serde_json::Value::Null => type_id.starts_with("Option<"),
        serde_json::Value::Bool(_) => type_id == "bool",
        serde_json::Value::Number(_) => matches!(
            type_id,
            "f64" | "f32" | "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16"
                | "u32" | "u64" | "u128"
        ),
        serde_json::Value::String(_) => type_id == "String" || type_id == "char",
        serde_json::Value::Array(_) => {
            type_id.starts_with("Vec<")
                || type_id.starts_with("Tuple<")
                || type_id.starts_with("Array<")
                || type_id == "Array"
        }
        serde_json::Value::Object(_) => {
            type_id.starts_with("Map<")
                || type_id == "Object"
                || (!type_id.starts_with("Vec<")
                    && !type_id.starts_with("Tuple<")
                    && !type_id.starts_with("Array<")
                    && !type_id.starts_with("Option<")
                    && !matches!(
                        type_id,
                        "bool" | "String" | "char" | "f64" | "f32" | "i8" | "i16" | "i32"
                            | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128"
                            | "unit" | "unknown"
                    ))
        }
    }
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
