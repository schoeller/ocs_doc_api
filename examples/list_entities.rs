use acadrust::entities::{Circle, EntityType, Line};
use ocs_doc_api::doc_api::{DocApiExt, Receipt};
use ocs_doc_api::in_process::InProcessDocApi;

fn main() {
    let mut api = InProcessDocApi::new("list_entities");

    let line = Line::from_coords(0.0, 0.0, 0.0, 5.0, 5.0, 0.0);
    api.create_entity(
        ocs_doc_api::doc_api::entity_to_payload(&EntityType::Line(line)).unwrap(),
    )
    .unwrap();

    let circle = Circle::from_coords(5.0, 5.0, 0.0, 2.0);
    api.create_entity(
        ocs_doc_api::doc_api::entity_to_payload(&EntityType::Circle(circle)).unwrap(),
    )
    .unwrap();

    let list = api.list_entities().unwrap();
    match list {
        Receipt::EntityList { entities } => {
            println!("document contains {} entities:", entities.len());
            for (handle, payload) in entities {
                println!("  handle {} -> {}", handle.0, payload.kind);
            }
        }
        other => panic!("unexpected receipt: {:?}", other),
    }
}
