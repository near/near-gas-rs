use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::NearGas;

impl Serialize for NearGas {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let mut buf = [0u8; 20];
        let remainder = {
            use std::io::Write;
            let mut w: &mut [u8] = &mut buf;
            write!(w, "{}", self.inner)
                .map_err(|err| Error::custom(format!("Failed to serialize: {}", err)))?;
            w.len()
        };
        let len = buf.len() - remainder;

        let s = std::str::from_utf8(&buf[..len])
            .map_err(|err| Error::custom(format!("Failed to serialize: {}", err)))?;
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for NearGas {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrNumberVisitor;

        impl serde::de::Visitor<'_> for StringOrNumberVisitor {
            type Value = NearGas;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a number")
            }

            fn visit_str<E>(self, value: &str) -> Result<NearGas, E>
            where
                E: serde::de::Error,
            {
                value
                    .parse::<u64>()
                    .map(NearGas::from_gas)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<NearGas, E>
            where
                E: serde::de::Error,
            {
                Ok(NearGas::from_gas(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<NearGas, E>
            where
                E: serde::de::Error,
            {
                let Ok(value_u64) = value.try_into() else {
                    return Err(serde::de::Error::custom(
                        "Negative value cannot be converted to NearGas",
                    ));
                };

                Ok(NearGas::from_gas(value_u64))
            }
        }

        deserializer.deserialize_any(StringOrNumberVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::NearGas;

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Wrapper {
        gas: NearGas,
    }

    #[test]
    fn json_ser() {
        fn test_json_ser(val: u64) {
            let gas = NearGas::from_gas(val);
            let ser = serde_json::to_string(&gas).unwrap();
            assert_eq!(ser, format!("\"{}\"", val));
            let de: NearGas = serde_json::from_str(&ser).unwrap();
            assert_eq!(de.as_gas(), val);
        }

        test_json_ser(u64::MAX);
        test_json_ser(8);
        test_json_ser(0);
    }

    #[test]
    fn json_deser_from_string_and_number() {
        let gas = serde_json::from_str::<NearGas>("\"100\"").unwrap();
        assert_eq!(gas.as_gas(), 100);
        let gas = serde_json::from_str::<NearGas>("100").unwrap();
        assert_eq!(gas.as_gas(), 100);
    }

    #[test]
    fn bson_ser() {
        fn test_bson_ser(val: u64) {
            let gas = NearGas::from_gas(val);
            let ser = bson::to_bson(&gas).unwrap();
            assert_eq!(ser, bson::Bson::String(format!("{}", val)));
            assert_eq!(ser.to_string(), format!("\"{}\"", val));
            let de: NearGas = bson::from_bson(ser).unwrap();
            assert_eq!(de.as_gas(), val);
        }

        test_bson_ser(u64::MAX);
        test_bson_ser(8);
        test_bson_ser(0);
    }

    #[test]
    fn bson_deser_from_string_and_number() {
        let gas = bson::from_bson::<NearGas>(bson::Bson::Int64(100)).unwrap();
        assert_eq!(gas.as_gas(), 100);
        let gas = bson::from_bson::<NearGas>(bson::Bson::String("100".to_string())).unwrap();
        assert_eq!(gas.as_gas(), 100);
    }

    #[test]
    fn bson_deser_from_bytes_integer() {
        let doc = bson::doc! { "gas": 10_i64 };
        let bytes = bson::to_vec(&doc).expect("serialize doc to BSON bytes");
        let decoded: Wrapper =
            bson::from_slice(&bytes).expect("deserialize from BSON bytes into Wrapper");
        assert_eq!(decoded.gas, NearGas::from_gas(10));
    }

    #[test]
    fn bson_deser_from_bytes_string() {
        let doc = bson::doc! { "gas": "10" };
        let bytes = bson::to_vec(&doc).expect("serialize doc to BSON bytes");
        let decoded: Wrapper =
            bson::from_slice(&bytes).expect("deserialize from BSON bytes into Wrapper");
        assert_eq!(decoded.gas, NearGas::from_gas(10));
    }

    #[test]
    fn bson_deser_from_bytes_negative_integer() {
        let doc = bson::doc! { "gas": -1_i64 };
        let bytes = bson::to_vec(&doc).expect("serialize doc to BSON bytes");
        let decoded: Result<Wrapper, _> = bson::from_slice(&bytes);
        assert!(
            decoded.is_err(),
            "deserializing negative gas value must fail"
        );
    }
    #[test]
    fn bson_deser_from_bytes_negative_string() {
        let doc = bson::doc! { "gas": "-1" };
        let bytes = bson::to_vec(&doc).expect("serialize doc to BSON bytes");
        let decoded: Result<Wrapper, _> = bson::from_slice(&bytes);
        assert!(
            decoded.is_err(),
            "deserializing negative gas value from string must fail"
        );
    }

    #[test]
    fn bson_deser_from_bytes_u64_max_string() {
        let doc = bson::doc! { "gas": u64::MAX.to_string() };
        let bytes = bson::to_vec(&doc).expect("serialize doc to BSON bytes");
        let decoded: Wrapper =
            bson::from_slice(&bytes).expect("deserialize u64::MAX from string into Wrapper");
        assert_eq!(
            decoded.gas,
            NearGas::from_gas(u64::MAX),
            "NearGas should handle u64::MAX when encoded as string"
        );
    }
}
