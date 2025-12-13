use fortifier::Validate;

#[derive(Validate)]
struct RegexData<'a> {
    #[validate(regex = "abc")]
    value: &'a str,
}

fn main() {}
