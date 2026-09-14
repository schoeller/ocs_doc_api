use serde_reflection::{ContainerFormat, Format, Registry, Samples, Tracer, TracerConfig};
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

// Build-time entity samples are shared with the runtime test crate.
#[path = "src/entity_samples.rs"]
mod entity_samples;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    // 1. Trace the acadrust object model. If serde-reflection cannot finish
    //    (recursive ACIS structures, unresolved optional boxed, etc.), fall
    //    back to an empty registry and back-fill every entity from JSON below.
    let mut registry = match trace_acadrust_registry() {
        Ok(reg) => reg,
        Err(e) => {
            eprintln!(
                "serde-reflection tracing incomplete ({}); using JSON-derived fallback only",
                e
            );
            schema::TypeRegistry {
                types: BTreeMap::new(),
            }
        }
    };

    // 2. Back-fill any entity shapes we intentionally keep out of serde-reflection
    //    (recursive ACIS structures) with coarse JSON-derived entries so every
    //    first-level `EntityType` variant still has a payload shape.
    supplement_entities_from_json(&mut registry);

    // 3. Promote a handful of well-known required fields so validation can
    //    reject payloads that omit them. The JSON-derived fallback above marks
    //    every field as optional because it cannot distinguish required from
    //    defaulted fields from a single sample.
    apply_known_required_fields(&mut registry);

    // 4. Merge kernel capabilities from the hand-written TOML file.
    let capabilities_path = Path::new(&manifest_dir).join("kernel_capabilities.toml");
    let capabilities = load_capabilities(&capabilities_path)?;
    merge_capabilities(&mut registry, capabilities);

    // 5. Load operation metadata for generated docs.
    let ops_path = Path::new(&manifest_dir).join("doc_api_ops.toml");
    let ops = load_doc_ops(&ops_path)?;

    // 6. Generate Markdown docs.
    let (object_model_md, ops_md) = generate_docs(&registry, &ops);

    // 7. Write JSON, dispatch code, and docs to OUT_DIR.
    let json_path = Path::new(&out_dir).join("object_model.json");
    // Compact JSON for the embedded copy; pretty Markdown is written below.
    std::fs::write(&json_path, serde_json::to_string(&registry)?)?;

    let dispatch_path = Path::new(&out_dir).join("object_model_dispatch.rs");
    std::fs::write(&dispatch_path, generate_dispatch(&registry))?;

    let object_model_docs_path = Path::new(&out_dir).join("object_model_docs.md");
    std::fs::write(&object_model_docs_path, &object_model_md)?;

    let doc_api_ops_path = Path::new(&out_dir).join("doc_api_ops.md");
    std::fs::write(&doc_api_ops_path, &ops_md)?;

    // Rerun-if-changed entries.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/entity_samples.rs");
    println!("cargo:rerun-if-changed=src/schema.rs");
    println!("cargo:rerun-if-changed=kernel_capabilities.toml");
    println!("cargo:rerun-if-changed=doc_api_ops.toml");

    Ok(())
}

/// Ensure that every first-level entity type has a registry entry by deriving
/// one from its JSON serialization. This covers entity shapes that we
/// intentionally keep out of serde-reflection (recursive ACIS structures) so
/// every variant still has a payload shape.
///
/// Fields are marked optional by default because the sample is only used for
/// shape discovery, not to declare requiredness.
fn supplement_entities_from_json(registry: &mut schema::TypeRegistry) {
    for entity in entity_samples::all() {
        let value = match serde_json::to_value(&entity) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let object = match value.as_object() {
            Some(o) if o.len() == 1 => o,
            _ => continue,
        };
        let (variant_name, data) = object.iter().next().unwrap();
        let type_name = variant_name.clone();
        if registry.types.contains_key(&type_name) {
            // Keep the more precise serde-reflection entry if one exists.
            continue;
        }
        let fields = match data.as_object() {
            Some(map) => map
                .iter()
                .map(|(k, v)| schema::FieldInfo {
                    name: k.clone(),
                    type_id: json_value_type_id(v),
                    required: false,
                })
                .collect(),
            None => Vec::new(),
        };
        registry.types.insert(
            type_name.clone(),
            schema::TypeInfo {
                kind: schema::TypeKind::Struct,
                name: type_name,
                rust_type: String::new(),
                fields,
                variants: Vec::new(),
                inner: None,
                len: None,
                key: None,
                value: None,
                capabilities: Vec::new(),
            },
        );
    }
}

fn json_value_type_id(value: &serde_json::Value) -> schema::TypeId {
    match value {
        serde_json::Value::Null => "Option<unknown>".into(),
        serde_json::Value::Bool(_) => "bool".into(),
        serde_json::Value::Number(_) => "f64".into(),
        serde_json::Value::String(_) => "String".into(),
        serde_json::Value::Array(_) => "Array".into(),
        serde_json::Value::Object(_) => "Object".into(),
    }
}

/// Promote known required fields that the JSON-derived fallback cannot
/// identify as required from a single sample.
fn apply_known_required_fields(registry: &mut schema::TypeRegistry) {
    let required_fields: std::collections::HashMap<&str, &[&str]> = [
        ("Line", &["start", "end"] as &[&str]),
        ("Circle", &["center", "radius"]),
        ("LwPolyline", &["vertices"]),
        ("Point", &["location"]),
        ("Text", &["text", "insertion_point"]),
        ("MText", &["text", "insertion_point"]),
    ]
    .into_iter()
    .collect();

    for (type_name, fields) in required_fields {
        if let Some(info) = registry.types.get_mut(type_name) {
            for field in &mut info.fields {
                if fields.contains(&field.name.as_str()) {
                    field.required = true;
                }
            }
        }
    }
}

fn trace_acadrust_registry() -> Result<schema::TypeRegistry, Box<dyn Error>> {
    use acadrust::entities::{Circle, EntityCommon, EntityType, Line, LwPolyline, Point};
    use acadrust::types::{
        Color, DxfVersion, Handle, LineWeight, Transparency, Vector2, Vector3,
    };
    use acadrust::xdata::XDataValue;

    let config = TracerConfig::default();
    let mut tracer = Tracer::new(config);
    let mut samples = Samples::new();

    // Register samples for shared value types so that tracing can use them.
    // Enums must have every variant sampled, otherwise serde-reflection refuses
    // to finalize the registry.
    tracer.trace_value(&mut samples, &Vector2::new(1.0, 2.0))?;
    tracer.trace_value(&mut samples, &Vector3::new(1.0, 2.0, 3.0))?;
    tracer.trace_value(&mut samples, &Color::ByLayer)?;
    tracer.trace_value(&mut samples, &Color::None)?;
    tracer.trace_value(&mut samples, &Color::ByBlock)?;
    tracer.trace_value(&mut samples, &Color::Index(1))?;
    tracer.trace_value(&mut samples, &Color::Rgb { r: 0, g: 0, b: 0 })?;
    tracer.trace_value(&mut samples, &Handle::new(0x1234))?;
    tracer.trace_value(&mut samples, &LineWeight::ByLayer)?;
    tracer.trace_value(&mut samples, &LineWeight::ByBlock)?;
    tracer.trace_value(&mut samples, &LineWeight::Default)?;
    tracer.trace_value(&mut samples, &LineWeight::Value(25))?;
    tracer.trace_value(&mut samples, &Transparency::ByLayer)?;
    tracer.trace_value(&mut samples, &Transparency::ByBlock)?;
    tracer.trace_value(&mut samples, &Transparency::Explicit(128))?;
    tracer.trace_value(&mut samples, &DxfVersion::Unknown)?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1012)?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1014)?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1015)?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1018)?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1021)?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1024)?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1027)?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1032)?;
    tracer.trace_value(&mut samples, &"sample".to_string())?;
    // XDataValue only appears inside optional extended data, but provide one
    // sample so the enum container can be resolved.
    tracer.trace_value(&mut samples, &XDataValue::String("s".into()))?;
    tracer.trace_value(&mut samples, &XDataValue::ControlString("{".into()))?;
    tracer.trace_value(&mut samples, &XDataValue::LayerName("L".into()))?;
    tracer.trace_value(&mut samples, &XDataValue::BinaryData(vec![0]))?;
    tracer.trace_value(&mut samples, &XDataValue::Handle(Handle::new(0)))?;
    tracer.trace_value(&mut samples, &XDataValue::Point3D(Vector3::new(0.0, 0.0, 0.0)))?;
    tracer.trace_value(&mut samples, &XDataValue::Position3D(Vector3::new(0.0, 0.0, 0.0)))?;
    tracer.trace_value(&mut samples, &XDataValue::Displacement3D(Vector3::new(0.0, 0.0, 0.0)))?;
    tracer.trace_value(&mut samples, &XDataValue::Direction3D(Vector3::new(1.0, 0.0, 0.0)))?;
    tracer.trace_value(&mut samples, &XDataValue::Real(0.0))?;
    tracer.trace_value(&mut samples, &XDataValue::Distance(0.0))?;
    tracer.trace_value(&mut samples, &XDataValue::ScaleFactor(0.0))?;
    tracer.trace_value(&mut samples, &XDataValue::Integer16(0))?;
    tracer.trace_value(&mut samples, &XDataValue::Integer32(0))?;

    // Register samples for the entity variants we want precise registry types
    // for. Every other first-level variant is still covered by the JSON-derived
    // fallback built from `entity_samples::all()`.
    tracer.trace_value(
        &mut samples,
        &EntityType::Line(Line::from_coords(0.0, 0.0, 0.0, 10.0, 0.0, 0.0)),
    )?;
    tracer.trace_value(
        &mut samples,
        &EntityType::Circle(Circle::from_coords(5.0, 5.0, 0.0, 2.0)),
    )?;
    tracer.trace_value(
        &mut samples,
        &EntityType::LwPolyline(LwPolyline::from_points(vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(10.0, 0.0),
            Vector2::new(10.0, 5.0),
        ])),
    )?;
    tracer.trace_value(&mut samples, &EntityType::Point(Point::from_coords(1.0, 2.0, 3.0)))?;

    // Trace shared common data. We intentionally omit document-level roots
    // (`CadDocument`, `HeaderVariables`) because they pull in the generic
    // symbol-table type `acadrust::tables::Table<T>`, whose short name collides
    // with the entity `Table`. The registry is focused on entity payloads.
    tracer.trace_simple_type::<EntityCommon>()?;

    let sr_registry = tracer.registry()?;
    Ok(map_registry(&sr_registry))
}

fn map_registry(sr_registry: &Registry) -> schema::TypeRegistry {
    let mut types = BTreeMap::new();
    for (full_name, container) in sr_registry.iter() {
        let name = strip_type_name(full_name);
        let info = map_container(name.clone(), container);
        types.insert(name, info);
    }
    schema::TypeRegistry { types }
}

fn strip_type_name(full: &str) -> String {
    full.split("::")
        .last()
        .unwrap_or(full)
        .to_string()
}

fn map_container(name: String, container: &ContainerFormat) -> schema::TypeInfo {
    match container {
        ContainerFormat::UnitStruct => schema::TypeInfo {
            kind: schema::TypeKind::UnitStruct,
            name,
            rust_type: String::new(),
            fields: Vec::new(),
            variants: Vec::new(),
            inner: None,
            len: None,
            key: None,
            value: None,
            capabilities: Vec::new(),
        },
        ContainerFormat::NewTypeStruct(inner) => schema::TypeInfo {
            kind: schema::TypeKind::NewTypeStruct,
            name,
            rust_type: String::new(),
            fields: Vec::new(),
            variants: Vec::new(),
            inner: Some(format_to_type_id(inner)),
            len: None,
            key: None,
            value: None,
            capabilities: Vec::new(),
        },
        ContainerFormat::TupleStruct(fields) => schema::TypeInfo {
            kind: schema::TypeKind::TupleStruct,
            name,
            rust_type: String::new(),
            fields: fields
                .iter()
                .enumerate()
                .map(|(i, f)| schema::FieldInfo {
                    name: format!("_{}", i),
                    type_id: format_to_type_id(f),
                    required: true,
                })
                .collect(),
            variants: Vec::new(),
            inner: None,
            len: None,
            key: None,
            value: None,
            capabilities: Vec::new(),
        },
        ContainerFormat::Struct(fields) => schema::TypeInfo {
            kind: schema::TypeKind::Struct,
            name,
            rust_type: String::new(),
            fields: fields
                .iter()
                .map(|named| schema::FieldInfo {
                    name: named.name.clone(),
                    type_id: format_to_type_id(&named.value),
                    required: !matches!(named.value, Format::Option(_)),
                })
                .collect(),
            variants: Vec::new(),
            inner: None,
            len: None,
            key: None,
            value: None,
            capabilities: Vec::new(),
        },
        ContainerFormat::Enum(variants) => schema::TypeInfo {
            kind: schema::TypeKind::Enum,
            name,
            rust_type: String::new(),
            fields: Vec::new(),
            variants: variants
                .iter()
                .map(|(index, named)| schema::EnumVariantInfo {
                    name: named.name.clone(),
                    index: *index,
                    fields: match &named.value {
                        serde_reflection::VariantFormat::Unit => Vec::new(),
                        serde_reflection::VariantFormat::NewType(f) => {
                            vec![schema::FieldInfo {
                                name: "value".into(),
                                type_id: format_to_type_id(f),
                                required: true,
                            }]
                        }
                        serde_reflection::VariantFormat::Tuple(fs) => fs
                            .iter()
                            .enumerate()
                            .map(|(i, f)| schema::FieldInfo {
                                name: format!("_{}", i),
                                type_id: format_to_type_id(f),
                                required: true,
                            })
                            .collect(),
                        serde_reflection::VariantFormat::Struct(fs) => fs
                            .iter()
                            .map(|n| schema::FieldInfo {
                                name: n.name.clone(),
                                type_id: format_to_type_id(&n.value),
                                required: !matches!(n.value, Format::Option(_)),
                            })
                            .collect(),
                        _ => Vec::new(),
                    },
                })
                .collect(),
            inner: None,
            len: None,
            key: None,
            value: None,
            capabilities: Vec::new(),
        },
    }
}

fn format_to_type_id(format: &Format) -> String {
    match format {
        Format::Variable(_) => "unknown".into(),
        Format::TypeName(name) => strip_type_name(name),
        Format::Unit => "unit".into(),
        Format::Bool => "bool".into(),
        Format::I8 => "i8".into(),
        Format::I16 => "i16".into(),
        Format::I32 => "i32".into(),
        Format::I64 => "i64".into(),
        Format::I128 => "i128".into(),
        Format::U8 => "u8".into(),
        Format::U16 => "u16".into(),
        Format::U32 => "u32".into(),
        Format::U64 => "u64".into(),
        Format::U128 => "u128".into(),
        Format::F32 => "f32".into(),
        Format::F64 => "f64".into(),
        Format::Char => "char".into(),
        Format::Str => "String".into(),
        Format::Bytes => "Vec<u8>".into(),
        Format::Option(inner) => format!("Option<{}>", format_to_type_id(inner)),
        Format::Seq(inner) => format!("Vec<{}>", format_to_type_id(inner)),
        Format::Map { key, value } => format!(
            "Map<{}, {}>",
            format_to_type_id(key),
            format_to_type_id(value)
        ),
        Format::Tuple(fs) => {
            let inner = fs.iter().map(format_to_type_id).collect::<Vec<_>>().join(", ");
            format!("Tuple<{}>", inner)
        }
        Format::TupleArray { content, size } => {
            format!("Array<{}; {}>", format_to_type_id(content), size)
        }
    }
}

#[derive(serde::Deserialize)]
struct CapabilitiesFile {
    #[serde(default)]
    capability: Vec<CapabilityEntry>,
}

#[derive(serde::Deserialize)]
struct CapabilityEntry {
    entity: String,
    name: String,
    output: String,
    #[serde(default)]
    requires: String,
}

fn load_capabilities(path: &Path) -> Result<Vec<(String, schema::Capability)>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path)?;
    let file: CapabilitiesFile = toml::from_str(&text)?;
    Ok(file
        .capability
        .into_iter()
        .map(|entry| {
            (
                entry.entity,
                schema::Capability {
                    name: entry.name,
                    output: entry.output,
                    requires: entry.requires,
                },
            )
        })
        .collect())
}

fn merge_capabilities(registry: &mut schema::TypeRegistry, caps: Vec<(String, schema::Capability)>) {
    for (entity, cap) in caps {
        let type_name = strip_type_name(&entity);
        if let Some(info) = registry.types.get_mut(&type_name) {
            info.capabilities.push(cap);
        } else {
            eprintln!("warning: capability references unknown entity type {}", entity);
        }
    }
}

#[derive(serde::Deserialize)]
struct OpsFile {
    #[serde(default)]
    op: Vec<OpEntry>,
}

#[derive(serde::Deserialize, Clone)]
struct OpEntry {
    name: String,
    description: String,
}

fn load_doc_ops(path: &Path) -> Result<Vec<OpEntry>, Box<dyn Error>> {
    let text = std::fs::read_to_string(path)?;
    let file: OpsFile = toml::from_str(&text)?;
    Ok(file.op)
}

fn generate_docs(
    registry: &schema::TypeRegistry,
    ops: &[OpEntry],
) -> (String, String) {
    // Rough capacity estimate: ~1.5 KiB per type keeps the buffer from
    // reallocating repeatedly while building the expanded object model docs.
    let mut object_model_md = String::with_capacity(registry.types.len() * 1536);
    object_model_md.push_str("# Object Model\n\n");
    for (name, info) in registry.types.iter() {
        object_model_md.push_str(&format!("## `{}`\n\n", name));
        object_model_md.push_str(&format!("Kind: `{:?}`\n\n", info.kind));
        if !info.fields.is_empty() {
            object_model_md.push_str("| Field | Type | Required |\n|---|---|---|\n");
            for field in &info.fields {
                object_model_md.push_str(&format!(
                    "| `{}` | `{}` | {} |\n",
                    field.name,
                    field.type_id,
                    if field.required { "yes" } else { "no" }
                ));
            }
            object_model_md.push('\n');
        }
        if !info.variants.is_empty() {
            object_model_md.push_str("| Variant | Fields |\n|---|---|\n");
            for variant in &info.variants {
                let fields: Vec<String> = variant
                    .fields
                    .iter()
                    .map(|f| format!("{}: {}", f.name, f.type_id))
                    .collect();
                object_model_md.push_str(&format!(
                    "| `{}` | {} |\n",
                    variant.name,
                    if fields.is_empty() {
                        "-".into()
                    } else {
                        fields.join(", ")
                    }
                ));
            }
            object_model_md.push('\n');
        }
        if !info.capabilities.is_empty() {
            object_model_md.push_str("Capabilities:\n");
            for cap in &info.capabilities {
                object_model_md.push_str(&format!(
                    "- `{}` → `{}`",
                    cap.name, cap.output
                ));
                if !cap.requires.is_empty() {
                    object_model_md.push_str(&format!(" (requires: {})", cap.requires));
                }
                object_model_md.push('\n');
            }
            object_model_md.push('\n');
        }
    }

    let mut ops_md = String::new();
    ops_md.push_str("# DocApi Operations\n\n");
    ops_md.push_str("| Operation | Description |\n|---|---|\n");
    for op in ops {
        ops_md.push_str(&format!("| `{}` | {} |\n", op.name, op.description));
    }

    (object_model_md, ops_md)
}

fn generate_dispatch(registry: &schema::TypeRegistry) -> String {
    fn escape_rust_str(s: &str, out: &mut String) {
        for ch in s.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c => out.push(c),
            }
        }
    }

    let mut code = String::new();
    code.push_str("// Generated by build.rs. Do not edit.\n");
    code.push_str("// Maps entity type names to kernel capabilities available at build time.\n\n");
    code.push_str("#[allow(dead_code)]\n");
    code.push_str("pub(crate) const KERNEL_CAPABILITIES: &[(&str, &[&str])] = &[\n");
    for (name, info) in registry.types.iter() {
        if info.capabilities.is_empty() {
            continue;
        }
        code.push_str("    (\"");
        escape_rust_str(name, &mut code);
        code.push_str("\", &[");
        let mut first = true;
        for cap in &info.capabilities {
            if !first {
                code.push_str(", ");
            }
            first = false;
            code.push('"');
            escape_rust_str(&cap.name, &mut code);
            code.push('"');
        }
        code.push_str("]),\n");
    }
    code.push_str("];\n");
    code
}

// Include the same schema definitions used by the runtime crate so the
// generated JSON shape matches exactly.
#[path = "src/schema.rs"]
mod schema;
