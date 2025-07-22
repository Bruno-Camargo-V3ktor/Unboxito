use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(Numbers),
    Str(String),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Bytes(Vec<u8>),
}

#[derive(Debug)]
pub enum Numbers {
    Int(i64),
    Float(f64),
}
