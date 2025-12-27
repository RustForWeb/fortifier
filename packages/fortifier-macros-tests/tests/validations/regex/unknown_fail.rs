use fortifier::Validate;

#[derive(Validate)]
struct UrlData<'a> {
    #[validate(regex(unknown))]
    value: &'a str,
}

fn main() {}
