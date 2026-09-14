use acadrust::entities::{EntityType, LwPolyline};
use acadrust::types::Vector2;
use ocs_doc_api::doc_api::{DocApiExt, Receipt};
use ocs_doc_api::in_process::InProcessDocApi;

fn main() {
    let mut api = InProcessDocApi::new("offset_polyline");

    let poly = LwPolyline::from_points(vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(10.0, 0.0),
        Vector2::new(10.0, 5.0),
    ]);
    let payload = ocs_doc_api::doc_api::entity_to_payload(&EntityType::LwPolyline(poly)).unwrap();

    let created = api.create_entity(payload).unwrap();
    let handle = match created {
        Receipt::EntityCreated { handle } => handle,
        other => panic!("unexpected receipt: {:?}", other),
    };

    let offset = api.offset_entity(handle, 0.5, Some([5.0, 2.5])).unwrap();
    match offset {
        Receipt::OffsetResult { results, .. } => {
            println!("offset produced {} result(s)", results.len());
            for (i, payload) in results.iter().enumerate() {
                println!("  result {}: kind={}", i, payload.kind);
            }
        }
        other => panic!("unexpected receipt: {:?}", other),
    }
}
