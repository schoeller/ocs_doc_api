use ocs_doc_api::doc_api::EntityPayload;
use ocs_doc_api::validate_entity_payload;
use serde_json::json;

fn main() {
    let registry = ocs_doc_api::object_model();

    // Valid line payload: all required fields are present.
    let valid = EntityPayload {
        kind: "line".into(),
        data: json!({
            "common": {},
            "start": { "x": 0.0, "y": 0.0, "z": 0.0 },
            "end": { "x": 1.0, "y": 0.0, "z": 0.0 },
            "thickness": 0.0,
            "normal": { "x": 0.0, "y": 0.0, "z": 1.0 }
        }),
    };
    match validate_entity_payload(&valid, &registry) {
        Ok(()) => println!("valid line payload accepted"),
        Err(e) => println!("valid payload rejected: {}", e),
    }

    // Invalid line payload: missing the required `start` field.
    let invalid = EntityPayload {
        kind: "line".into(),
        data: json!({
            "common": {},
            "end": { "x": 1.0, "y": 0.0, "z": 0.0 },
            "thickness": 0.0,
            "normal": { "x": 0.0, "y": 0.0, "z": 1.0 }
        }),
    };
    match validate_entity_payload(&invalid, &registry) {
        Ok(()) => println!("invalid payload unexpectedly accepted"),
        Err(e) => println!("invalid payload correctly rejected: {}", e),
    }
}
