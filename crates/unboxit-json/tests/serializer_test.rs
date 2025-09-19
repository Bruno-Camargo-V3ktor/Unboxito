use unboxit::{Serialize, Serializer};
use unboxit_json::JsonSerializer;

#[test]
fn test_serialize_bool() {
    let serializer = JsonSerializer::new();
    let result = serializer.serialize_bool(true).unwrap();
    assert_eq!(result, "true");

    let serializer = JsonSerializer::new();
    let result = serializer.serialize_bool(false).unwrap();
    assert_eq!(result, "false");
}

#[test]
fn test_serialize_numbers() {
    let s = JsonSerializer::new();
    let result = s.serialize_i64(12345).unwrap();
    assert_eq!(result, "12345");

    let s = JsonSerializer::new();
    let result = s.serialize_i64(-500).unwrap();
    assert_eq!(result, "-500");

    let s = JsonSerializer::new();
    let result = s.serialize_u64(0).unwrap();
    assert_eq!(result, "0");

    let s = JsonSerializer::new();
    let result = s.serialize_f64(3.14).unwrap();
    assert_eq!(result, "3.14");

    let s = JsonSerializer::new();
    let result = s.serialize_f64(f64::NAN);
    assert!(result.is_err());

    let s = JsonSerializer::new();
    let result = s.serialize_f64(f64::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_serialize_string() {
    let s = JsonSerializer::new();
    let result = s.serialize_str("hello world").unwrap();
    assert_eq!(result, r#""hello world""#);

    let s = JsonSerializer::new();
    let result = s.serialize_str("").unwrap();
    assert_eq!(result, r#""""#);

    let s = JsonSerializer::new();
    let result = s.serialize_str("a \"b\\c\nd").unwrap();
    assert_eq!(result, r#""a \"b\\c\nd""#);
}

#[test]
fn test_serialize_option() {
    let serializer = JsonSerializer::new();
    let some_val = Some(123);
    assert_eq!(some_val.serialize(serializer).unwrap(), "123");

    let serializer = JsonSerializer::new();
    let some_val: Option<i32> = None;
    assert_eq!(some_val.serialize(serializer).unwrap(), "null");

    let serializer = JsonSerializer::new();
    let some_val = Some("hello");
    assert_eq!(some_val.serialize(serializer).unwrap(), r#""hello""#);
}

#[test]
fn test_serializer_vetor() {
    let v = vec![10, 20, 30];
    let serializer = JsonSerializer::new();
    assert_eq!(v.serialize(serializer).unwrap(), "[10,20,30]");

    let v_str: Vec<&str> = vec!["a", "b"];
    let serializer = JsonSerializer::new();
    assert_eq!(v_str.serialize(serializer).unwrap(), r#"["a","b"]"#);

    let v_empty: Vec<i32> = vec![];
    let serializer = JsonSerializer::new();
    assert_eq!(v_empty.serialize(serializer).unwrap(), "[]");
}
