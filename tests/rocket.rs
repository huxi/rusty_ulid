use rocket::request::FromParam;
use rusty_ulid::Ulid;

#[test]
fn test_from_param() {
    let ulid_str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
    let ulid = Ulid::from_param(ulid_str).unwrap();
    assert_eq!(ulid_str, ulid.to_string());
}

#[test]
fn test_from_param_invalid() {
    let ulid_str = "01ARZ3NDEKTSV4RRFFQ69G5FAU";
    assert!(Ulid::from_param(ulid_str).is_err());
}
