use unboxit::Serializer;
use unboxit_json::JsonSerializer;

#[test]
fn test_serialize_bool() {
    let mut serializer = JsonSerializer::new();
    serializer.serialize_bool(true).unwrap();
    assert_eq!(serializer.into_inner(), "true");

    let mut serializer = JsonSerializer::new();
    serializer.serialize_bool(false).unwrap();
    assert_eq!(serializer.into_inner(), "false");
}
