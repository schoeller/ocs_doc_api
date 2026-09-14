#![cfg(feature = "engine")]

use acadrust::entities::{EntityType, Line, LwPolyline};
use acadrust::types::Vector2;
use ocs_doc_api::{
    doc_api::{entity_to_payload, DocApiExt, EntityPayload},
    in_process::InProcessDocApi,
    validate_entity_payload,
};

fn make_lwpolyline_payload() -> EntityPayload {
    let poly = LwPolyline::from_points(vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(10.0, 0.0),
        Vector2::new(10.0, 5.0),
    ]);
    entity_to_payload(&EntityType::LwPolyline(poly)).unwrap()
}

#[test]
fn validate_rejects_missing_required_field() {
    let registry = ocs_doc_api::object_model();
    let line = Line::from_coords(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let mut payload = entity_to_payload(&EntityType::Line(line)).unwrap();
    // Remove a required field from the inner data object.
    if let Some(obj) = payload.data.as_object_mut() {
        obj.remove("start");
    }
    let err = validate_entity_payload(&payload, &registry).unwrap_err();
    assert!(format!("{}", err).contains("start"));
}

#[test]
fn in_process_polyline_create_and_offset() {
    let mut api = InProcessDocApi::new("test");
    let create = api.create_entity(make_lwpolyline_payload()).unwrap();
    let handle = match create {
        ocs_doc_api::doc_api::Receipt::EntityCreated { handle } => handle,
        _ => panic!("expected EntityCreated"),
    };
    let offset = api.offset_entity(handle, 0.5, Some([5.0, 2.5])).unwrap();
    match offset {
        ocs_doc_api::doc_api::Receipt::OffsetResult { results, .. } => {
            assert!(!results.is_empty());
            for payload in &results {
                assert_eq!(payload.kind, "lwpolyline");
            }
        }
        _ => panic!("expected OffsetResult"),
    }
}

#[test]
fn doc_api_ops_toml_matches_doc_op_variants() {
    let manifest_dir = std::env!("CARGO_MANIFEST_DIR");
    let text = std::fs::read_to_string(std::path::Path::new(manifest_dir).join("doc_api_ops.toml"))
        .unwrap();
    let file: toml::Value = text.parse().unwrap();
    let names: Vec<String> = file
        .get("op")
        .and_then(|v| v.as_array())
        .unwrap()
        .iter()
        .map(|t| t.get("name").unwrap().as_str().unwrap().to_owned())
        .collect();
    ocs_doc_api::doc_api::validate_doc_op_names(&names).unwrap();
}
