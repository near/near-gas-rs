use crate::NearGas;
use crate::trait_impls::schemars_exports::schemars;

#[cfg(feature = "schemars-v0_8")]
impl schemars::JsonSchema for NearGas {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

#[cfg(feature = "schemars-v1")]
impl schemars::JsonSchema for NearGas {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NearGas".to_string().into()
    }

    fn json_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(gen)
    }
}
