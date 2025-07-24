use crate::NearGas;
use crate::trait_impls::schemars_exports::schemars;

const JS_MAX_SAFE_INTEGER: u64 = (1u64 << 53) - 1;

#[cfg(feature = "schemars-v0_8")]
impl schemars::JsonSchema for NearGas {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        u64::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        u64::json_schema(gen)
    }
}

#[cfg(feature = "schemars-v1")]
impl schemars::JsonSchema for NearGas {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NearGas".to_string().into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "integer",
            "format": "uint64",
            "minimum": 0,
            "maximum": JS_MAX_SAFE_INTEGER,
        })
    }
}
