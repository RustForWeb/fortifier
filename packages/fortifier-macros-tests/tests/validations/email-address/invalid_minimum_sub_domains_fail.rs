use fortifier::Validate;

#[derive(Validate)]
struct EmailAddressData<'a> {
    #[validate(email_address(minimum_sub_domains = -1))]
    value: &'a str,
}

fn main() {}
