use fortifier::Validate;

#[derive(Validate)]
struct EmailData<'a> {
    #[validate(email(allow_domain_literal = 1))]
    value: &'a str,
}

fn main() {}
