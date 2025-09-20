use unboxit::Serialize;
use unboxit_derive::UnboxitSerialize;
use unboxit_json::JsonSerializer;

#[test]
fn derive_struct_pointer() {
    #[derive(UnboxitSerialize)]
    struct Pointer {
        x: i32,
        y: i32,
    }

    let s = JsonSerializer::new();
    let pointer = Pointer { x: 0, y: 0 };
    let json_pointer = pointer.serialize(s).unwrap();
    assert_eq!(json_pointer, r#"{"x":0,"y":0}"#);
}
