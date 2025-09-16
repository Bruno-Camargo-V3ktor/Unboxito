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
}
