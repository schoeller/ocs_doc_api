use acadrust::entities::{EntityType, Line};
use ocs_doc_api::doc_api::{DocApiExt, Receipt};
use ocs_doc_api::in_process::InProcessDocApi;

fn main() {
    let mut api = InProcessDocApi::new("create_and_read_line");

    let line = Line::from_coords(0.0, 0.0, 0.0, 10.0, 5.0, 0.0);
    let payload = ocs_doc_api::doc_api::entity_to_payload(&EntityType::Line(line)).unwrap();

    let created = api.create_entity(payload).unwrap();
    let handle = match created {
        Receipt::EntityCreated { handle } => handle,
        other => panic!("unexpected receipt: {:?}", other),
    };
    println!("created line with handle {}", handle.0);

    let read = api.read_entity(handle).unwrap();
    println!("{:#?}", read);
}
