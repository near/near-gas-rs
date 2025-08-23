use crate::trait_impls::schemars_exports::schemars;
use crate::NearGas;

#[cfg(feature = "schemars-v0_8")]
impl schemars::JsonSchema for NearGas {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        "NearGas".to_string()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
        })
    }
}

#[cfg(feature = "schemars-v1")]
impl schemars::JsonSchema for NearGas {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NearGas".to_string().into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
        })
    }
}

#[cfg(test)]
mod test {
    use crate::trait_impls::schemars_exports::schemars;
    use crate::NearGas;
    use serde_json::json;

    #[test]
    #[cfg(feature = "schemars-v0_8")]
    fn json_schema_json_eq_v0_8() {
        let root = schemars::schema_for!(NearGas);
        let schema_json = serde_json::to_value(&root.schema).unwrap();
        assert_eq!(schema_json, json!({ "title": "NearGas", "type": "string" }));
    }

    #[test]
    #[cfg(feature = "schemars-v1")]
    fn json_schema_json_eq_v1() {
        let root = schemars::schema_for!(NearGas);
        let schema_json = serde_json::to_value(&root).unwrap();
        assert_eq!(
            schema_json,
            json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "title": "NearGas", "type": "string" })
        );
    }
}
