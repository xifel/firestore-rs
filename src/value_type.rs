use crate::serde_native_serializer::FirestoreValueSerializer;
use gcloud_sdk::google::firestore::v1::Value;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone)]
pub struct FirestoreValue {
    pub value: Value,
}

impl FirestoreValue {
    pub fn from(value: Value) -> Self {
        Self { value }
    }
}

impl<T> std::convert::From<T> for FirestoreValue
where
    T: Serialize,
{
    fn from(value: T) -> Self {
        let serializer = FirestoreValueSerializer {};
        value
            .serialize(serializer)
            .unwrap_or(FirestoreValue::from(Value { value_type: None }))
    }
}
