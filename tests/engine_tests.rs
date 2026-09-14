#![cfg(feature = "engine")]

use acadrust::entities::{EntityType, Line, LwPolyline};
use acadrust::types::Vector2;
use ocs_doc_api::{
    doc_api::{entity_to_payload, DocApiExt, EntityPayload, Receipt},
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

/// Compare two entity data JSON values, ignoring `handle`/`owner_handle`
/// inside any `common` object recursively.
fn data_equal_without_handles(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    match (a, b) {
        (serde_json::Value::Object(a_obj), serde_json::Value::Object(b_obj)) => {
            if a_obj.len() != b_obj.len() {
                return false;
            }
            for (key, a_val) in a_obj {
                let Some(b_val) = b_obj.get(key) else {
                    return false;
                };
                if key == "common" {
                    if let (Some(a_common), Some(b_common)) =
                        (a_val.as_object(), b_val.as_object())
                    {
                        let a_filtered: serde_json::Map<String, serde_json::Value> = a_common
                            .iter()
                            .filter(|(k, _)| *k != "handle" && *k != "owner_handle")
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect();
                        let b_filtered: serde_json::Map<String, serde_json::Value> = b_common
                            .iter()
                            .filter(|(k, _)| *k != "handle" && *k != "owner_handle")
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect();
                        if a_filtered != b_filtered {
                            return false;
                        }
                        continue;
                    }
                }
                if !data_equal_without_handles(a_val, b_val) {
                    return false;
                }
            }
            true
        }
        (serde_json::Value::Array(a_arr), serde_json::Value::Array(b_arr)) => {
            a_arr.len() == b_arr.len()
                && a_arr
                    .iter()
                    .zip(b_arr.iter())
                    .all(|(a, b)| data_equal_without_handles(a, b))
        }
        _ => a == b,
    }
}

/// Create an entity payload and round-trip it through `InProcessDocApi`.
fn round_trip_entity(entity: EntityType) {
    let payload = entity_to_payload(&entity).unwrap();
    let expected_kind = payload.kind.clone();

    let registry = ocs_doc_api::object_model();
    validate_entity_payload(&payload, &registry).unwrap();

    let mut api = InProcessDocApi::new("test");
    let handle = match api.create_entity(payload.clone()).unwrap() {
        Receipt::EntityCreated { handle } => handle,
        other => panic!("expected EntityCreated, got {:?}", other),
    };

    let read = match api.read_entity(handle).unwrap() {
        Receipt::EntityRead { payload, .. } => payload,
        other => panic!("expected EntityRead, got {:?}", other),
    };
    assert_eq!(read.kind, expected_kind);
    assert!(
        data_equal_without_handles(&read.data, &payload.data),
        "round-trip data mismatch for {}",
        expected_kind
    );
}

#[test]
fn round_trip_all_entities() {
    for entity in ocs_doc_api::entity_samples::all() {
        round_trip_entity(entity);
    }
}

#[test]
fn validate_rejects_unknown_field() {
    let registry = ocs_doc_api::object_model();
    let line = Line::from_coords(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let mut payload = entity_to_payload(&EntityType::Line(line)).unwrap();
    if let Some(obj) = payload.data.as_object_mut() {
        obj.insert("extra".into(), serde_json::json!(42));
    }
    let err = validate_entity_payload(&payload, &registry).unwrap_err();
    assert!(format!("{}", err).contains("extra"));
}

#[test]
fn validate_rejects_type_mismatch() {
    let registry = ocs_doc_api::object_model();
    let line = Line::from_coords(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let mut payload = entity_to_payload(&EntityType::Line(line)).unwrap();
    if let Some(obj) = payload.data.as_object_mut() {
        obj.insert("thickness".into(), serde_json::json!("thick"));
    }
    let err = validate_entity_payload(&payload, &registry).unwrap_err();
    assert!(format!("{}", err).contains("thickness"));
}

#[test]
fn validate_rejects_missing_required_field() {
    let registry = ocs_doc_api::object_model();
    let line = Line::from_coords(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let mut payload = entity_to_payload(&EntityType::Line(line)).unwrap();
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
        Receipt::EntityCreated { handle } => handle,
        _ => panic!("expected EntityCreated"),
    };
    let offset = api.offset_entity(handle, 0.5, Some([5.0, 2.5])).unwrap();
    match offset {
        Receipt::OffsetResult { results, .. } => {
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
