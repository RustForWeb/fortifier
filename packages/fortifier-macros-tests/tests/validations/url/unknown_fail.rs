use fortifier::Validate;

#[derive(Validate)]
struct UrlData<'a> {
    #[validate(url(unknown))]
    value: &'a str,
}

fn main() {}
