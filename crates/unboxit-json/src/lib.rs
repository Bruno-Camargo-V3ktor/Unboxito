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

    fn serialize_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        if v {
            self.output.push_str("true");
        } else {
            self.output.push_str("false");
        }

        Ok(())
    }

    fn serialize_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        if v.is_finite() {
            self.output.push_str(&v.to_string());
            Ok(())
        } else {
            Err(Error::Message(
                "Cannot serialize non-finite f64".to_string(),
            ))
        }
    }
}
