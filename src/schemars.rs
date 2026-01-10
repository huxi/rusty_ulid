use std::borrow::Cow;

/// Provides integration for JsonSchema based data annotation.
use crate::Ulid;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;

impl JsonSchema for Ulid {
    fn schema_name() -> Cow<'static, str> {
        "Ulid".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "title": "[ULID](https://github.com/ulid/spec)",
            "description": "[Universally Unique Lexicographically Sortable Identifier](https://github.com/ulid/spec)",
            "type": "string",
            "format": "ulid",
            "examples": [
                "01ARZ3NDEKTSV4RRFFQ69G5FAV",
                "01BX5ZZKBKACTAV9WEVGEMMVS0"
            ]
        })
    }
}
