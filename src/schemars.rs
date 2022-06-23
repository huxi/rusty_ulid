/// Provides integration for JsonSchema based data annotation.
use crate::Ulid;
use schemars::gen::SchemaGenerator;
use schemars::schema::*;
use schemars::JsonSchema;

impl JsonSchema for Ulid {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        "Ulid".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            metadata: Some(Box::new(Metadata {
                title: Some("[ULID](https://github.com/ulid/spec)".to_string()),
                description: Some("[Universally Unique Lexicographically Sortable Identifier](https://github.com/ulid/spec)".to_string()),
                examples: ["01ARZ3NDEKTSV4RRFFQ69G5FAV", "01BX5ZZKBKACTAV9WEVGEMMVS0"].map(|v| v.to_string().into()).to_vec(),
                ..Default::default()
            })),
            instance_type: Some(InstanceType::String.into()),
            format: Some("ulid".to_string()),
            ..Default::default()
        }
        .into()
    }
}
