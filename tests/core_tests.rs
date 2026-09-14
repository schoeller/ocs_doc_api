use ocs_doc_api::schema::TypeRegistry;

#[test]
fn embedded_json_parses_into_registry() {
    let registry = ocs_doc_api::object_model();
    assert!(!registry.type_names().is_empty());
}

#[test]
fn line_has_to_planar_curve_capability() {
    let caps = ocs_doc_api::capabilities_for("Line");
    assert!(caps.iter().any(|c| c.name == "to_planar_curve"));
}

#[test]
fn lwpolyline_has_offset_capability() {
    let caps = ocs_doc_api::capabilities_for("LwPolyline");
    assert!(caps.iter().any(|c| c.name == "offset"));
}

#[test]
fn registry_json_round_trips() {
    let registry = ocs_doc_api::object_model();
    let json = serde_json::to_string(&registry).unwrap();
    let parsed: TypeRegistry = serde_json::from_str(&json).unwrap();
    assert_eq!(registry, parsed);
}

#[test]
fn embedded_markdown_docs_are_non_empty() {
    assert!(!ocs_doc_api::embedded_object_model_docs_md().is_empty());
    assert!(!ocs_doc_api::embedded_doc_api_ops_md().is_empty());
}
