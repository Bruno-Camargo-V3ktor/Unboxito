use unboxit::{Serializer, error::Error};

pub struct JsonSerializer {
    output: String,
}

impl JsonSerializer {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    pub fn into_inner(self) -> String {
        self.output
    }
}

impl Serializer for JsonSerializer {
    type Ok = String;
    type Error = Error;

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
}
