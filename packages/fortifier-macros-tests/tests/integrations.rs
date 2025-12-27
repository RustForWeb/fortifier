use trybuild::TestCases;

#[test]
fn integrations() {
    let t = TestCases::new();
    t.pass("tests/integrations/*_pass.rs");
    t.compile_fail("tests/integrations/*_fail.rs");
}
