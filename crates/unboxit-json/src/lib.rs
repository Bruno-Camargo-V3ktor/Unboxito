use unboxit::{Serialize, SerializeSeq, SerializeStruct, Serializer, error::Error};

pub struct JsonSerializer {}

pub struct JsonSeqSerializer {
    output: String,
    is_first: bool,
}

pub struct JsonStructSerializer {
    output: String,
    is_first: bool,
}

impl JsonSerializer {
    pub fn new() -> Self {
        Self {}
    }
}

impl SerializeSeq for JsonSeqSerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: unboxit::Serialize,
    {
        if !self.is_first {
            self.output.push(',');
        }
        self.is_first = false;

        let element_str = value.serialize(JsonSerializer::new())?;
        self.output.push_str(&element_str);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.output.push(']');
        Ok(self.output)
    }
}

impl SerializeStruct for JsonStructSerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: unboxit::Serialize,
    {
        if !self.is_first {
            self.output.push(',');
        }
        self.is_first = false;

        let key_str = key.serialize(JsonSerializer::new())?;
        self.output.push_str(&key_str);

        self.output.push(':');

        let value_str = value.serialize(JsonSerializer::new())?;
        self.output.push_str(&value_str);

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.output.push('}');
        Ok(self.output)
    }
}

impl Serializer for JsonSerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = JsonSeqSerializer;
    type SerializeStruct = JsonStructSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if v.is_finite() {
            Ok(v.to_string())
        } else {
            Err(Error::Message(
                "Cannot serialize non-finite f64".to_string(),
            ))
        }
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let mut s = String::with_capacity(v.len() + 4);
        s.push('"');

        for c in v.chars() {
            match c {
                '"' => s.push_str("\\\""),
                '\\' => s.push_str("\\\\"),
                '\n' => s.push_str("\\n"),
                '\r' => s.push_str("\\r"),
                '\t' => s.push_str("\\t"),

                _ => s.push(c),
            }
        }

        s.push('"');
        Ok(s)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok("null".to_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(JsonSeqSerializer {
            output: "[".to_string(),
            is_first: true,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(JsonStructSerializer {
            is_first: true,
            output: "{".to_string(),
        })
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(name)
    }
}
