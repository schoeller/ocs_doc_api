use serde_reflection::{ContainerFormat, Format, Registry, Samples, Tracer, TracerConfig};
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var("OUT_DIR")?;

    // 1. Trace the acadrust object model.
    let registry = match trace_acadrust_registry() {
        Ok(reg) => reg,
        Err(e) => {
            eprintln!(
                "serde-reflection tracing failed ({}); using minimal fallback registry",
                e
            );
            fallback_registry()
        }
    };

    // 2. Merge kernel capabilities from the hand-written TOML file.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let capabilities_path = Path::new(&manifest_dir).join("kernel_capabilities.toml");
    let capabilities = load_capabilities(&capabilities_path)?;
    let mut registry = registry;
    merge_capabilities(&mut registry, capabilities);

    // 3. Load operation metadata for generated docs.
    let ops_path = Path::new(&manifest_dir).join("doc_api_ops.toml");
    let ops = load_doc_ops(&ops_path)?;

    // 4. Generate Markdown docs.
    let (object_model_md, ops_md) = generate_docs(&registry, &ops);

    // 5. Write JSON, dispatch code, and docs to OUT_DIR.
    let json_path = Path::new(&out_dir).join("object_model.json");
    std::fs::write(&json_path, serde_json::to_string_pretty(&registry)?)?;

    let dispatch_path = Path::new(&out_dir).join("object_model_dispatch.rs");
    std::fs::write(&dispatch_path, generate_dispatch(&registry))?;

    let object_model_docs_path = Path::new(&out_dir).join("object_model_docs.md");
    std::fs::write(&object_model_docs_path, &object_model_md)?;

    let doc_api_ops_path = Path::new(&out_dir).join("doc_api_ops.md");
    std::fs::write(&doc_api_ops_path, &ops_md)?;

    // Rerun-if-changed entries.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=kernel_capabilities.toml");
    println!("cargo:rerun-if-changed=doc_api_ops.toml");

    Ok(())
}

fn trace_acadrust_registry() -> Result<schema::TypeRegistry, Box<dyn Error>> {
    use acadrust::document::HeaderVariables;
    use acadrust::entities::{Circle, EntityCommon, EntityType, Line, LwPolyline, MText, Point};
    use acadrust::objects::ObjectType;
    use acadrust::types::{Color, DxfVersion, Handle, LineWeight, Transparency, Vector2, Vector3};
    use acadrust::CadDocument;

    let config = TracerConfig::default();
    let mut tracer = Tracer::new(config);
    let mut samples = Samples::new();

    // Register samples for shared value types so that tracing can use them.
    tracer.trace_value(&mut samples, &Vector2::new(1.0, 2.0))?;
    tracer.trace_value(&mut samples, &Vector3::new(1.0, 2.0, 3.0))?;
    tracer.trace_value(&mut samples, &Color::RED)?;
    tracer.trace_value(&mut samples, &Handle::new(0x1234))?;
    tracer.trace_value(&mut samples, &LineWeight::default())?;
    tracer.trace_value(&mut samples, &Transparency::default())?;
    tracer.trace_value(&mut samples, &DxfVersion::AC1032)?;

    // Register samples for entity variants we want to expose. Providing one
    // sample per variant lets serde-reflection discover the variant shape.
    tracer.trace_value(&mut samples, &EntityType::Line(Line::from_coords(0.0, 0.0, 0.0, 10.0, 0.0, 0.0)))?;
    tracer.trace_value(&mut samples, &EntityType::Circle(Circle::from_coords(5.0, 5.0, 0.0, 2.0)))?;
    tracer.trace_value(
        &mut samples,
        &EntityType::LwPolyline(LwPolyline::from_points(vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(10.0, 0.0),
            Vector2::new(10.0, 5.0),
        ])),
    )?;
    tracer.trace_value(&mut samples, &EntityType::MText(MText::new()))?;
    tracer.trace_value(&mut samples, &EntityType::Point(Point::from_coords(1.0, 2.0, 3.0)))?;
    tracer.trace_value(&mut samples, &EntityCommon::new())?;

    // Trace the main roots. `ObjectType` is a large enum; tracing it here
    // lets the registry include non-graphical objects as well.
    tracer.trace_simple_type::<CadDocument>()?;
    tracer.trace_simple_type::<HeaderVariables>()?;
    tracer.trace_simple_type::<EntityType>()?;
    tracer.trace_simple_type::<EntityCommon>()?;
    tracer.trace_simple_type::<ObjectType>()?;

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
    let mut object_model_md = String::new();
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
    let mut code = String::new();
    code.push_str("// Generated by build.rs. Do not edit.\n");
    code.push_str("// Maps entity type names to kernel capabilities available at build time.\n\n");
    code.push_str("#[allow(dead_code)]\n");
    code.push_str("pub(crate) const KERNEL_CAPABILITIES: &[(&str, &[&str])] = &[\n");
    for (name, info) in registry.types.iter() {
        if info.capabilities.is_empty() {
            continue;
        }
        let cap_names: Vec<String> = info
            .capabilities
            .iter()
            .map(|c| format!("\"{}\"", c.name))
            .collect();
        code.push_str(&format!(
            "    (\"{}\", &[{}]),\n",
            name,
            cap_names.join(", ")
        ));
    }
    code.push_str("];\n");
    code
}

fn fallback_registry() -> schema::TypeRegistry {
    let mut types = BTreeMap::new();
    types.insert(
        "Line".into(),
        schema::TypeInfo {
            kind: schema::TypeKind::Struct,
            name: "Line".into(),
            rust_type: String::new(),
            fields: vec![
                schema::FieldInfo {
                    name: "common".into(),
                    type_id: "EntityCommon".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "start".into(),
                    type_id: "Vector3".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "end".into(),
                    type_id: "Vector3".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "thickness".into(),
                    type_id: "f64".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "normal".into(),
                    type_id: "Vector3".into(),
                    required: true,
                },
            ],
            variants: Vec::new(),
            inner: None,
            len: None,
            key: None,
            value: None,
            capabilities: Vec::new(),
        },
    );
    types.insert(
        "LwPolyline".into(),
        schema::TypeInfo {
            kind: schema::TypeKind::Struct,
            name: "LwPolyline".into(),
            rust_type: String::new(),
            fields: vec![
                schema::FieldInfo {
                    name: "common".into(),
                    type_id: "EntityCommon".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "vertices".into(),
                    type_id: "Vec<LwVertex>".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "is_closed".into(),
                    type_id: "bool".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "plinegen".into(),
                    type_id: "bool".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "constant_width".into(),
                    type_id: "f64".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "elevation".into(),
                    type_id: "f64".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "thickness".into(),
                    type_id: "f64".into(),
                    required: true,
                },
                schema::FieldInfo {
                    name: "normal".into(),
                    type_id: "Vector3".into(),
                    required: true,
                },
            ],
            variants: Vec::new(),
            inner: None,
            len: None,
            key: None,
            value: None,
            capabilities: Vec::new(),
        },
    );
    schema::TypeRegistry { types }
}

// Include the same schema definitions used by the runtime crate so the
// generated JSON shape matches exactly.
#[path = "src/schema.rs"]
mod schema;
