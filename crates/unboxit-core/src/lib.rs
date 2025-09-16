pub mod error {
    use std::fmt::{self, Display};

    #[derive(Debug)]
    pub enum Error {
        Message(String),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::Message(msg) => write!(f, "{}", msg),
            }
        }
    }

    impl std::error::Error for Error {}
}

pub trait Serializer {
    type Ok;
    type Error: std::error::Error;

    fn serialize_bool(&mut self, v: bool) -> Result<(), Self::Error>;
    fn serialize_i64(&mut self, v: i64) -> Result<(), Self::Error>;
    fn serialize_u64(&mut self, v: u64) -> Result<(), Self::Error>;
    fn serialize_f64(&mut self, v: f64) -> Result<(), Self::Error>;
}

pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

pub trait Deserializer<'de> {
    type Error: std::error::Error;
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
