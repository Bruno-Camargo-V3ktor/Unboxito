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

#[test]
fn test_serialize_numbers() {
    let mut s = JsonSerializer::new();
    s.serialize_i64(12345).unwrap();
    assert_eq!(s.into_inner(), "12345");

    let mut s = JsonSerializer::new();
    s.serialize_i64(-500).unwrap();
    assert_eq!(s.into_inner(), "-500");

    let mut s = JsonSerializer::new();
    s.serialize_u64(0).unwrap();
    assert_eq!(s.into_inner(), "0");

    let mut s = JsonSerializer::new();
    s.serialize_f64(3.14).unwrap();
    assert_eq!(s.into_inner(), "3.14");

    let mut s = JsonSerializer::new();
    let result = s.serialize_f64(f64::NAN);
    assert!(result.is_err());

    let mut s = JsonSerializer::new();
    let result = s.serialize_f64(f64::INFINITY);
    assert!(result.is_err());
}
