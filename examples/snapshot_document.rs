use ocs_doc_api::doc_api::Receipt;
use ocs_doc_api::in_process::InProcessDocApi;
use ocs_doc_api::DocApiExt;

fn main() {
    let mut api = InProcessDocApi::new("snapshot_document");

    // Add a simple line.
    let line = acadrust::entities::Line::from_coords(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
    api.create_entity(
        ocs_doc_api::doc_api::entity_to_payload(&acadrust::entities::EntityType::Line(line))
            .unwrap(),
    )
    .unwrap();

    let snapshot = api.get_document().unwrap();
    match snapshot {
        Receipt::DocumentSnapshot(doc) => {
            println!("document name: {}", doc.name);
            println!("entity count: {}", doc.entities.len());
            for payload in doc.entities {
                println!("  entity kind: {}", payload.kind);
            }
        }
        other => panic!("unexpected receipt: {:?}", other),
    }
}
