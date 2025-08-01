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
        }

        deserializer.deserialize_any(StringOrNumberVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::NearGas;
    use crate::trait_impls::serde::JS_MAX_SAFE_INTEGER;

    #[test]
    fn json_ser() {
        fn test_json_ser(val: u64) {
            let gas = NearGas::from_gas(val);
            let ser = serde_json::to_string(&gas).unwrap();
            assert_eq!(ser, format!("{}", val));
            let de: NearGas = serde_json::from_str(&ser).unwrap();
            assert_eq!(de.as_gas(), val);
        }

        test_json_ser(JS_MAX_SAFE_INTEGER);
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
}
