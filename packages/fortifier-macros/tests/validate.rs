use trybuild::TestCases;

#[test]
fn validate() {
    let t = TestCases::new();
    t.pass("tests/validate/*_pass.rs");
    t.compile_fail("tests/validate/*_fail.rs");
}
