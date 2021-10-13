extern crate rustex;

#[test]
fn test_say_hi_contains_name() {
    let name = "Proful";
    assert!(rustex::say_hi(name).contains(name));
}
