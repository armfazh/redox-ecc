use super::{private_version, version};

#[test]
fn uint_testing() {
    assert_eq!(version(), "0.1.0");
    assert_eq!(version(), private_version());
}
