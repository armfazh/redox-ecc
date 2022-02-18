use redox_ecc::version;

#[test]
fn integration_testing() {
    assert_eq!(version(), "0.2.3");
}
