use unboxit::{Serializer, error::Error};

pub struct JsonSerializer {
    output: String,
}

impl Serializer for JsonSerializer {
    type Ok = String;
    type Error = Error;
}
