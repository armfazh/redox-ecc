use super::{private_version, version};

#[test]
fn uint_testing() {
    assert_eq!(version(), "0.2.1");
    assert_eq!(version(), private_version());
}
