use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser};
use serde_with::{DeserializeAs, SerializeAs, serde_as};

// /// Serializes an [`ObjectId`] as a hex string. (from bson repo)
// fn serialize_object_id_as_hex_string<S: serde::Serializer>(
//     val: &ObjectId,
//     serializer: S,
// ) -> Result<S::Ok, S::Error> {
//     val.to_hex().serialize(serializer)
// }

// // Suggested workaround from Isabel (github issue)
// fn serialize_object_id_option_as_hex_string<S: serde::Serializer>(
//     val: &Option<ObjectId>,
//     serializer: S,
// ) -> Result<S::Ok, S::Error> {
//     match val {
//         Some(oid) => oid.to_hex().serialize(serializer),
//         None => serializer.serialize_none(),
//     }
// }

// /// Serializes an `ObjectId` as a hex string, supporting compatibility with:
// /// - `ObjectId`
// /// - `Option<ObjectId>`
// /// - `Vec<ObjectId>`
// pub fn serialize_object_id_generic<S, T>(val: &T, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
//     T: SerializeObjectId,
// {
//     val.serialize_as_hex(serializer)
// }

// /// A trait to unify serialization logic for ObjectId, Option<ObjectId>, and Vec<ObjectId>
// pub trait SerializeObjectId {
//     fn serialize_as_hex<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer;
// }

// /// Implementation for a single `ObjectId`
// impl SerializeObjectId for ObjectId {
//     fn serialize_as_hex<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         self.to_hex().serialize(serializer)
//     }
// }

// /// Implementation for `Option<ObjectId>`
// impl SerializeObjectId for Option<ObjectId> {
//     fn serialize_as_hex<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             Some(oid) => oid.to_hex().serialize(serializer),
//             None => serializer.serialize_none(),
//         }
//     }
// }

// /// Implementation for `Vec<ObjectId>`
// impl SerializeObjectId for Vec<ObjectId> {
//     fn serialize_as_hex<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(self.len()))?;
//         for oid in self {
//             seq.serialize_element(&oid.to_hex())?;
//         }
//         seq.end()
//     }
// }

/// Adapter for serializing and deserializing `ObjectId` as hex strings using `serde_with`  
pub struct ObjectIdAsHexString;

impl SerializeAs<ObjectId> for ObjectIdAsHexString {
    fn serialize_as<S>(val: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.to_hex().serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, ObjectId> for ObjectIdAsHexString {
    fn deserialize_as<D>(deserializer: D) -> Result<ObjectId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_string = String::deserialize(deserializer)?;
        ObjectId::parse_str(&hex_string).map_err(serde::de::Error::custom)
    }
}
/// Adapter for serializing and deserializing `ObjectId` as hex strings using `serde_with`  
pub struct HexStringAsObjectId;

impl SerializeAs<String> for HexStringAsObjectId {
    fn serialize_as<S>(val: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // val.to_hex().serialize(serializer)
        match ObjectId::parse_str(val) {
            Ok(oid) => oid.serialize(serializer),
            Err(_) => Err(ser::Error::custom(format!(
                "cannot convert {} to ObjectId",
                val
            ))),
        }
    }
}

impl<'de> DeserializeAs<'de, String> for HexStringAsObjectId {
    fn deserialize_as<D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        // let hex_string = String::deserialize(deserializer)?;
        // ObjectId::parse_str(&hex_string).map_err(serde::de::Error::custom)
        let object_id = ObjectId::deserialize(deserializer)?;
        Ok(object_id.to_hex())
    }
}

// pub mod hex_string_as_object_id {
//     use serde::{Deserialize, Deserializer, Serialize, Serializer, ser};

//     /// Deserializes a hex string from an ObjectId.
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let object_id = ObjectId::deserialize(deserializer)?;
//         Ok(object_id.to_hex())
//     }

//     /// Serializes a hex string as an ObjectId.
//     pub fn serialize<S: Serializer>(val: &str, serializer: S) -> Result<S::Ok, S::Error> {
//         match ObjectId::parse_str(val) {
//             Ok(oid) => oid.serialize(serializer),
//             Err(_) => Err(ser::Error::custom(format!(
//                 "cannot convert {} to ObjectId",
//                 val
//             ))),
//         }
//     }
// }

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct ObjectIdAndString {
    #[serde_as(as = "ObjectIdAsHexString")]
    pub id: ObjectId,

    #[serde_as(as = "Option<ObjectIdAsHexString>")]
    pub optional_id: Option<ObjectId>,

    #[serde_as(as = "Vec<ObjectIdAsHexString>")]
    pub id_vec: Vec<ObjectId>,

    // #[serde(with = "hex_string_as_object_id")]
    // pub hex_string: String,

    // #[serde(with = "Option<hex_string_as_object_id>")]
    // pub optional_string: Option<String>,

    // #[serde(with = "Vec<hex_string_as_object_id>")]
    // pub string_vec: Vec<String>,
    #[serde_as(as = "HexStringAsObjectId")]
    pub hex_string: String,

    #[serde_as(as = "Option<HexStringAsObjectId>")]
    pub opt_string: Option<String>,

    #[serde_as(as = "Vec<HexStringAsObjectId>")]
    pub string_vec: Vec<String>,
}

pub fn run_example() {
    let oid = ObjectId::new();
    let foo = ObjectIdAndString {
        id: oid,
        optional_id: Some(oid),
        id_vec: vec![ObjectId::new(), ObjectId::new()],
        hex_string: oid.to_hex(),
        opt_string: Some(oid.to_hex()),
        string_vec: vec![oid.to_hex(), ObjectId::new().to_hex()],
    };

    let serialized = serde_json::to_string(&foo).unwrap();
    println!("Serialized ObjectIdAndString:\t\t\t {}", serialized);
    let deserialized: ObjectIdAndString = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized ObjectIdAndString:\t\t {:?}", deserialized);
}
