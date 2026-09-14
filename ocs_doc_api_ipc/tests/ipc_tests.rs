use ocs_doc_api::{
    doc_api::{DocApi, DocApiExt},
    in_process::InProcessDocApi,
};
use ocs_doc_api_ipc::{decode_op, encode_receipt, LocalDocApiClient};

#[test]
fn local_callback_round_trip_create_and_offset() {
    let mut host_doc = InProcessDocApi::new("host");

    // Host-side dispatch closure.
    let mut dispatch = |tab_id: u64, bytes: &[u8]| -> Vec<u8> {
        assert_eq!(tab_id, 1);
        let op = decode_op(bytes).unwrap();
        let result = host_doc.execute(op);
        encode_receipt(result).unwrap()
    };

    let mut client = LocalDocApiClient::new(1, &mut dispatch);

    // Build a payload from a real lwpolyline.
    let poly = acadrust::entities::LwPolyline::from_points(vec![
        acadrust::types::Vector2::new(0.0, 0.0),
        acadrust::types::Vector2::new(10.0, 0.0),
        acadrust::types::Vector2::new(10.0, 5.0),
    ]);
    let payload = ocs_doc_api::doc_api::entity_to_payload(
        &acadrust::entities::EntityType::LwPolyline(poly),
    )
    .unwrap();

    let create = client.create_entity(payload).unwrap();
    let handle = match create {
        ocs_doc_api::doc_api::Receipt::EntityCreated { handle } => handle,
        _ => panic!("expected EntityCreated"),
    };

    let offset = client.offset_entity(handle, 0.5, Some([5.0, 2.5])).unwrap();
    match offset {
        ocs_doc_api::doc_api::Receipt::OffsetResult { results, .. } => {
            assert!(!results.is_empty());
        }
        _ => panic!("expected OffsetResult"),
    }
}
