use unboxit::{ Serialize, SerializeStruct, Serializer, SerializeTupleVariant };
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

#[test]
fn test_serialize_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    impl Serialize for Point {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            let mut state = serializer.serialize_struct("Point", 2)?;
            state.serialize_field("x", &self.x)?;
            state.serialize_field("y", &self.y)?;
            state.end()
        }
    }

    let p = Point { x: 1, y: -10 };
    let serializer = JsonSerializer::new();
    let expected = r#"{"x":1,"y":-10}"#;
    assert_eq!(p.serialize(serializer).unwrap(), expected);
}

use unboxit_derive::UnboxitSerialize;

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

#[test]
fn derive_struct_rename_pointer() {
    #[derive(UnboxitSerialize)]
    struct Pointer {
        #[unboxit(rename = "cord_x")]
        x: i32,
        #[unboxit(rename = "cord_y")]
        y: i32,
    }

    let s = JsonSerializer::new();
    let pointer = Pointer { x: 0, y: 0 };
    let json_pointer = pointer.serialize(s).unwrap();
    assert_eq!(json_pointer, r#"{"cord_x":0,"cord_y":0}"#);
}

#[test]
fn derive_struct_unit() {
    #[derive(UnboxitSerialize)]
    struct MyStruct;

    let s = JsonSerializer::new();
    let my_struct = MyStruct;
    let json = my_struct.serialize(s).unwrap();
    assert_eq!(json, r#""MyStruct""#);
}

#[test]
fn derive_struct_tuple() {
    #[derive(UnboxitSerialize)]
    struct TupleStruct(String, bool, i32);

    let s = JsonSerializer::new();
    let tuple_struct = TupleStruct("hello".to_string(), true, 123);
    let json = tuple_struct.serialize(s).unwrap();
    assert_eq!(json, r#"["hello",true,123]"#);
}

#[test]
fn test_enum_variant_serialization() {
    let s = JsonSerializer::new();
    let result = s.serialize_unit_variant("Status", 0, "Ativo").unwrap();
    assert_eq!(result, r#""Ativo""#);

    let s = JsonSerializer::new();
    let result = s.serialize_newtype_variant("Comando", 1, "DefinirUsuario", &123i32).unwrap();
    assert_eq!(result, r#"{"DefinirUsuario":123}"#);

    let s = JsonSerializer::new();
    let mut helper = s.serialize_tuple_variant("Cor", 0, "Rgb", 2).unwrap();

    helper.serialize_field(&255i32).unwrap();
    helper.serialize_field(&100i32).unwrap();

    let result = helper.end().unwrap();
    assert_eq!(result, r#"{"Rgb":[255,100]}"#);

    let s = JsonSerializer::new();
    let mut helper = s.serialize_struct_variant("Evento", 0, "Click", 2).unwrap();

    helper.serialize_field("x", &10i32).unwrap();
    helper.serialize_field("y", &20i32).unwrap();

    let result = helper.end().unwrap();
    assert_eq!(result, r#"{"Click":{"x":10,"y":20}}"#);
}
