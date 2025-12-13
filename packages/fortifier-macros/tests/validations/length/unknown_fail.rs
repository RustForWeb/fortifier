use fortifier::Validate;

#[derive(Validate)]
struct LengthData<'a> {
    #[validate(length(unknown = 2))]
    value: &'a str,
}

fn main() {}
