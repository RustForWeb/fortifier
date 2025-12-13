use fortifier::Validate;

#[derive(Validate)]
struct EmailData<'a> {
    #[validate(email(unknown = true))]
    value: &'a str,
}

fn main() {}
