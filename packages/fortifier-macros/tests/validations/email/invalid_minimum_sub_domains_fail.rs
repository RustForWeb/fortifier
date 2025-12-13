use fortifier::Validate;

#[derive(Validate)]
struct EmailData<'a> {
    #[validate(email(minimum_sub_domains = -1))]
    value: &'a str,
}

fn main() {}
