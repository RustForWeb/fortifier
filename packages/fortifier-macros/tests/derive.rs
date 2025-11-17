use trybuild::TestCases;

#[test]
fn derive() {
    let t = TestCases::new();
    t.pass("tests/derive/*_pass.rs");
    t.compile_fail("tests/derive/*_fail.rs");
}
