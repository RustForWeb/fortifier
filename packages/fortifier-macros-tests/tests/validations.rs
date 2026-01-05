use trybuild::TestCases;

#[test]
fn validations() {
    let t = TestCases::new();
    t.pass("tests/validations/*/*_pass.rs");
    t.compile_fail("tests/validations/*/*_fail.rs");
}
