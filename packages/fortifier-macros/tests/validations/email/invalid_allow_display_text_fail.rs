use fortifier::Validate;

#[derive(Validate)]
struct EmailData<'a> {
    #[validate(email(allow_display_text = 1))]
    value: &'a str,
}

fn main() {}
