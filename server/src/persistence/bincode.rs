use bincode::config;
use redb::{TypeName, Value};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Bincode<T>(pub T);

impl<T> Value for Bincode<T>
where
    T: Debug + Serialize + for<'a> Deserialize<'a>,
{
    type SelfType<'a>
        = T
    where
        Self: 'a;
    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> T
    where
        Self: 'a,
    {
        let (val, _len) = bincode::serde::decode_from_slice(data, config::standard())
            .expect("Bincode deserialization corrupted");
        val
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a T) -> Vec<u8>
    where
        Self: 'b,
    {
        let bytes: Vec<u8> = bincode::serde::encode_to_vec(value, config::standard())
            .expect("Impossible bincode serialization");
        bytes
    }

    fn type_name() -> TypeName {
        TypeName::new(&format!("Bincode<{}>", std::any::type_name::<T>()))
    }
}
