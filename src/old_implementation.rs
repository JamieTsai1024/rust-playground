use mongodb::bson::oid::ObjectId;
// use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

fn serialize_object_id_option_as_hex_string<S: serde::Serializer>(
    val: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        Some(oid) => oid.to_hex().serialize(serializer),
        None => serializer.serialize_none(),
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct FooOldUndesired {
    #[serde(
        // Behaviour 1: Deserializes id as _id: { oid: "..." } instead of id: "..."
        skip_serializing_if = "Option::is_none",
        // Behaviour 2: Using serialize_with doesn't work - customer request to make it work
        // skip_serializing_if = "Option::is_none",
        // serialize_with = "serialize_object_id_as_hex_string",
    )]
    id: Option<ObjectId>,
}

#[derive(Deserialize, Serialize, Debug)]
struct FooOld {
    #[serde(
        // Behaviour 3: Workaround, custom helper function for serialization 
        serialize_with = "serialize_object_id_option_as_hex_string"
    )]
    id: Option<ObjectId>,
}

pub fn run_example() {
    let oid = ObjectId::new();

    // Original serialization
    let foo = FooOldUndesired { id: Some(oid) };
    let serialized = serde_json::to_string(&foo).unwrap();
    println!("Old Serialized [Undesired]:\t {}", serialized);
    // Deserialize won't work for Behviour 2
    let deserialized: FooOldUndesired = serde_json::from_str(&serialized).unwrap();
    println!("Old Deserialized [Undesired]:\t {:?}", deserialized);

    // Workaround serialization with custom helper
    let oid = ObjectId::new();
    let foo = FooOld { id: Some(oid) };
    let serialized = serde_json::to_string(&foo).unwrap();
    println!("Old Serialized [Workaround]:\t {}", serialized);
    let deserialized: FooOld = serde_json::from_str(&serialized).unwrap();
    println!("Old Deserialized [Workaround]:\t {:?}", deserialized);
}
