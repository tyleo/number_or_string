mod serialization {
    use serde::de::{ Deserialize, Deserializer, Error, Visitor };
    use serde::ser::{ Serialize, Serializer };
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum NumberOrString {
        Number(usize),
        String(String),
    }

    impl Deserialize for NumberOrString {
        fn deserialize<D>(deserializer: &mut D) -> Result<NumberOrString, D::Error>
            where D: Deserializer {
                struct NumberOrStringVisitor;

                impl Visitor for NumberOrStringVisitor {
                    type Value = NumberOrString;

                    fn visit_u64<E>(&mut self, value: u64) -> Result<NumberOrString, E>
                        where E: Error {
                        Ok(NumberOrString::Number(value as usize))
                    }

                    fn visit_usize<E>(&mut self, value: usize) -> Result<NumberOrString, E>
                        where E: Error {
                        Ok(NumberOrString::Number(value))
                    }

                    fn visit_str<E>(&mut self, value: &str) -> Result<NumberOrString, E>
                        where E: Error {
                        Ok(NumberOrString::String(value.to_string()))
                    }
                }

                deserializer.deserialize(NumberOrStringVisitor)
        }
    }

    impl Serialize for NumberOrString {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: Serializer {
            match self {
                &NumberOrString::Number(value) => serializer.serialize_usize(value),
                &NumberOrString::String(ref value) => serializer.serialize_str(value),
            }
        }
    }
}
