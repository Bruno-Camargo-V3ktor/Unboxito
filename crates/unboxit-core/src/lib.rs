pub mod error {
    use std::fmt::{ self, Display };

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
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleVariant: SerializeTupleVariant<Ok = Self::Ok, Error = Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize
    ) -> Result<Self::SerializeStruct, Self::Error>;

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str
    ) -> Result<Self::Ok, Self::Error>;

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T
    ) -> Result<Self::Ok, Self::Error>;

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize
    ) -> Result<Self::SerializeStruct, Self::Error>;
}

pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer;
}

pub trait Deserializer<'de> {
    type Error: std::error::Error;
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>;
}

////////////////////////////////

pub trait SerializeSeq {
    type Ok;
    type Error: std::error::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait SerializeStruct {
    type Ok;
    type Error: std::error::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T
    ) -> Result<(), Self::Error>
        where T: Serialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait SerializeTupleVariant {
    type Ok;
    type Error: std::error::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: Serialize;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

////////////////////////////////

impl<T> Serialize for Vec<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut seq_helper = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq_helper.serialize_element(element)?;
        }

        seq_helper.end()
    }
}

impl<T> Serialize for Option<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Some(v) => v.serialize(serializer),
            None => serializer.serialize_unit(),
        }
    }
}

impl Serialize for bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_bool(*self)
    }
}

impl Serialize for i32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_i64(*self as i64)
    }
}

impl Serialize for &str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self)
    }
}

impl Serialize for String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self)
    }
}
