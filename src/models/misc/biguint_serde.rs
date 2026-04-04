//! Shared serde helpers for `BigUint` fields (decimal string representation).

pub(crate) mod decimal_biguint {
    use num_bigint::BigUint;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    #[derive(Deserialize)]
    #[serde(untagged)]
    pub enum Repr {
        String(String),
        U64(u64),
        I64(i64),
    }

    pub fn parse_repr<E: Error>(value: Repr) -> Result<BigUint, E> {
        match value {
            Repr::String(s) => BigUint::parse_bytes(s.as_bytes(), 10)
                .ok_or_else(|| E::custom(format!("invalid decimal integer: {s}"))),
            Repr::U64(n) => Ok(BigUint::from(n)),
            Repr::I64(n) if n >= 0 => Ok(BigUint::from(n as u64)),
            Repr::I64(n) => Err(E::custom(format!("expected nonnegative integer, got {n}"))),
        }
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
        parse_repr(Repr::deserialize(deserializer)?)
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
        let values = Vec::<super::decimal_biguint::Repr>::deserialize(deserializer)?;
        values
            .into_iter()
            .map(super::decimal_biguint::parse_repr::<D::Error>)
            .collect()
    }
}
