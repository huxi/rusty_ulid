use pretty_assertions::assert_eq;
use rusty_ulid::Ulid;
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use std::error::Error;
use std::fs;

#[test]
fn ulid() -> TestResult {
    test_default_generated_schema::<Ulid>("ulid")
}

type TestResult = Result<(), Box<dyn Error>>;

#[allow(dead_code)] // https://github.com/rust-lang/rust/issues/46379
pub fn test_default_generated_schema<T: JsonSchema>(file: &str) -> TestResult {
    let actual = schema_for!(T);
    test_schema(&actual, file)
}

fn test_schema(actual: &RootSchema, file: &str) -> TestResult {
    let expected_json = match fs::read_to_string(format!("tests/expected/{file}.json")) {
        Ok(j) => j,
        Err(e) => {
            write_actual_to_file(actual, file)?;
            return Err(Box::from(e));
        }
    };
    let expected = &serde_json::from_str(&expected_json)?;

    if actual != expected {
        write_actual_to_file(actual, file)?;
    }

    assert_eq!(expected, actual);
    Ok(())
}

fn write_actual_to_file(schema: &RootSchema, file: &str) -> TestResult {
    let actual_json = serde_json::to_string_pretty(&schema)?;
    fs::write(format!("tests/actual/{file}.json"), actual_json)?;
    Ok(())
}
