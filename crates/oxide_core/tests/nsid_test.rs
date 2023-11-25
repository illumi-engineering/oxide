use std::str::FromStr;
use oxide_core::NSID;

#[test]
fn test_nsid_plus_string() {
    let base_id = NSID::from_parts(vec![String::from("oxide")]);
    let new_id = base_id + String::from("test");
    assert_eq!(new_id, NSID::from_str("oxide:test").unwrap())
}

#[test]
fn test_nsid_plus_str() {
    let base_id = NSID::from_parts(vec![String::from("oxide")]);
    let new_id = base_id + "test";
    assert_eq!(new_id, NSID::from_str("oxide:test").unwrap())
}

#[test]
fn test_nsid_plus_nsid() {
    let base_id_1 = NSID::from_str("oxide:plugins").unwrap();
    let base_id_2 = NSID::from_str("packaging:npm").unwrap();
    let new_id = base_id_1 + base_id_2;
    assert_eq!(new_id, NSID::from_str("oxide:plugins:packaging:npm").unwrap())
}