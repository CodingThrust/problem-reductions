//! Shared serde helpers for `BigUint` fields (decimal string representation).

pub(crate) mod decimal_biguint {
    use num_bigint::BigUint;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn parse<E: Error>(value: &str) -> Result<BigUint, E> {
        BigUint::parse_bytes(value.as_bytes(), 10)
            .ok_or_else(|| E::custom(format!("invalid decimal integer: {value}")))
    }

    pub fn serialize<S>(value: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_str_radix(10))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
    where
        D: Deserializer<'de>,
    {
        parse(&String::deserialize(deserializer)?)
    }
}

pub(crate) mod decimal_biguint_vec {
    use num_bigint::BigUint;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(values: &[BigUint], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let strings: Vec<String> = values.iter().map(ToString::to_string).collect();
        strings.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BigUint>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = Vec::<String>::deserialize(deserializer)?;
        values
            .into_iter()
            .map(|value| super::decimal_biguint::parse::<D::Error>(&value))
            .collect()
    }
}
