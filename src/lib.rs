use std::collections::HashMap;

pub enum Value {
    Null,
    Boolean(bool),
    Number(Numbers),
    Str(String),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Bytes(Vec<u8>),
}

pub enum Numbers {
    Int(i64),
    Float(f64),
}
