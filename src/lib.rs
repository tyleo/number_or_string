#![recursion_limit="200"]

extern crate serde;

use serde::de::{ Deserialize, Deserializer, Error, Visitor };
use serde::ser::{ Serialize, Serializer };
use std::fmt::{ Formatter, Result as FmtResult };

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NumberOrString {
    Number(u64),
    String(String),
}

impl <'a> Deserialize<'a> for NumberOrString {
    fn deserialize<D>(deserializer: D) -> Result<NumberOrString, D::Error>
        where D: Deserializer<'a> {
            struct NumberOrStringVisitor;

            impl <'b> Visitor<'b> for NumberOrStringVisitor {
                type Value = NumberOrString;

                fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                    formatter.write_str("A u64 or a string.")
                }

                fn visit_u32<E>(self, value: u32) -> Result<NumberOrString, E>
                    where E: Error {
                    Ok(NumberOrString::Number(value as u64))
                }

                fn visit_u64<E>(self, value: u64) -> Result<NumberOrString, E>
                    where E: Error {
                    Ok(NumberOrString::Number(value))
                }

                fn visit_str<E>(self, value: &str) -> Result<NumberOrString, E>
                    where E: Error {
                    Ok(NumberOrString::String(value.to_string()))
                }
            }

            deserializer.deserialize_any(NumberOrStringVisitor)
    }
}

impl Serialize for NumberOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        match self {
            &NumberOrString::Number(value) => serializer.serialize_u64(value),
            &NumberOrString::String(ref value) => serializer.serialize_str(value),
        }
    }
}
