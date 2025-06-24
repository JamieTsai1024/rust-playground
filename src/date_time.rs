use mongodb::bson::{
    Bson, DateTime,
    serde_helpers::{
        deserialize_bson_datetime_from_rfc3339_string, serialize_bson_datetime_as_rfc3339_string,
    },
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser};
use serde_with::{DeserializeAs, SerializeAs, serde_as};
use std::result::Result;

/// Adapter for serializing and deserializing `ObjectId` as hex strings using `serde_with`  
// pub struct ObjectIdAsHexString;

// impl SerializeAs<ObjectId> for ObjectIdAsHexString {
//     fn serialize_as<S>(val: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         val.to_hex().serialize(serializer)
//     }
// }

// impl<'de> DeserializeAs<'de, ObjectId> for ObjectIdAsHexString {
//     fn deserialize_as<D>(deserializer: D) -> Result<ObjectId, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let hex_string = String::deserialize(deserializer)?;
//         ObjectId::parse_str(&hex_string).map_err(serde::de::Error::custom)
//     }
// }

pub struct BsonDatetimeAsRfc3339String;

impl SerializeAs<DateTime> for BsonDatetimeAsRfc3339String {
    fn serialize_as<S>(val: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // val.to_hex().serialize(serializer)
        let formatted = val
            .try_to_rfc3339_string()
            .map_err(|e| ser::Error::custom(format!("cannot format {} as RFC 3339: {}", val, e)))?;
        serializer.serialize_str(&formatted)
    }
}

impl<'de> DeserializeAs<'de, DateTime> for BsonDatetimeAsRfc3339String {
    fn deserialize_as<D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        // let hex_string = String::deserialize(deserializer)?;
        // crate::DateTime::parse_str(&hex_string).map_err(serde::de::Error::custom)
        let iso = String::deserialize(deserializer)?;
        let date = DateTime::parse_rfc3339_str(&iso).map_err(|_| {
            de::Error::custom(format!("cannot parse RFC 3339 datetime from \"{}\"", iso))
        })?;
        Ok(date)
    }
}
pub struct Rfc3339StringAsBsonDatetime;
// use crate::{Bson, DateTime};
// use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
// use std::result::Result;

impl SerializeAs<String> for Rfc3339StringAsBsonDatetime {
    fn serialize_as<S>(val: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // val.to_hex().serialize(serializer)
        // let formatted = val
        //     .try_to_rfc3339_string()
        //     .map_err(|e| ser::Error::custom(format!("cannot format {} as RFC 3339: {}", val, e)))?;
        // serializer.serialize_str(&formatted)
        let date = DateTime::parse_rfc3339_str(val)
            .map_err(|_| ser::Error::custom(format!("cannot convert {} to DateTime", val)))?;
        Bson::DateTime(date).serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, String> for Rfc3339StringAsBsonDatetime {
    fn deserialize_as<D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        // let hex_string = String::deserialize(deserializer)?;
        // crate::DateTime::parse_str(&hex_string).map_err(serde::de::Error::custom)
        // let iso = String::deserialize(deserializer)?;
        // let date = DateTime::parse_rfc3339_str(&iso).map_err(|_| {
        //     de::Error::custom(format!("cannot parse RFC 3339 datetime from \"{}\"", iso))
        // })?;
        // Ok(date)
        let date = DateTime::deserialize(deserializer)?;
        date.try_to_rfc3339_string()
            .map_err(|e| de::Error::custom(format!("cannot format {} as RFC 3339: {}", date, e)))
    }
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct DateTimeAndString {
    #[serde_as(as = "BsonDatetimeAsRfc3339String")]
    pub date: DateTime,

    #[serde_as(as = "Option<BsonDatetimeAsRfc3339String>")]
    pub optional_date: Option<DateTime>,

    #[serde_as(as = "Vec<BsonDatetimeAsRfc3339String>")]
    pub date_vec: Vec<DateTime>,

    // Goal to replace (either alternative works)
    // #[serde(with = "bson_datetime_as_rfc3339_string")]
    // #[serde(
    //     deserialize_with = "deserialize_bson_datetime_from_rfc3339_string",
    //     serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    // )]
    // pub updated_at: DateTime,
    // #[serde(
    //     deserialize_with = "deserialize_bson_datetime_from_rfc3339_string",
    //     serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    // )]
    // pub opt_date: Option<DateTime>,
    // pub vec_date: Vec<DateTime>,
    #[serde_as(as = "Rfc3339StringAsBsonDatetime")]
    pub string: String,

    #[serde_as(as = "Option<Rfc3339StringAsBsonDatetime>")]
    pub optional_string: Option<String>,

    #[serde_as(as = "Vec<Rfc3339StringAsBsonDatetime>")]
    pub string_vec: Vec<String>,
}

pub fn run_example() {
    let date = DateTime::now();
    let date_str = "2020-06-09T10:58:07.095Z";
    let foo = DateTimeAndString {
        date: date,
        optional_date: None,
        date_vec: vec![DateTime::now(), DateTime::now()],
        string: date_str.to_string(),
        // string: DateTime::from_chrono(date), // date.to_string(),
        optional_string: Some(date_str.to_string()),
        string_vec: vec![date_str.to_string(), date_str.to_string()],
    };

    // let date = chrono::DateTime::<chrono::Utc>::from_str(iso).unwrap();
    // let a = A {
    //     date: crate::DateTime::from_chrono(date),
    // };

    let serialized = serde_json::to_string(&foo).unwrap();
    println!("Serialized DateTimeAndString:\t\t\t {}", serialized);
    let deserialized: DateTimeAndString = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized DateTimeAndString:\t\t {:?}", deserialized);
}
