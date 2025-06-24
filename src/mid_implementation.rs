use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

// This is how serialize_with works with serde_with!
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct FooMid {
    // Serialize/Deserialize ObjectId as RFC 3339 using DisplayFromStr
    #[serde_as(as = "DisplayFromStr")]
    pub id: ObjectId,

    // Optional ObjectId field
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub optional_id: Option<ObjectId>,

    // Vector of ObjectId fields
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub id_vec: Vec<ObjectId>,
}

pub fn run_example() {
    let oid = ObjectId::new();
    let foo = FooMid {
        id: oid,
        optional_id: Some(oid),
        id_vec: vec![ObjectId::new(), ObjectId::new()],
    };

    // Serialize
    let serialized = serde_json::to_string(&foo).unwrap();
    println!("Mid Serialized:\t\t\t {}", serialized);

    // Deserialize (works because `DisplayFromStr` supports both `to_hex` and `FromStr`)
    let deserialized: FooMid = serde_json::from_str(&serialized).unwrap();
    println!("Mid Deserialized:\t\t {:?}", deserialized);
}
