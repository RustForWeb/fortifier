use fortifier::Validate;

#[derive(Validate)]
struct LengthData<'a> {
    #[validate(length(equal = 1, min = 2))]
    value: &'a str,
}

fn main() {}
