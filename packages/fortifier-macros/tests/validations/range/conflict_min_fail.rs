use fortifier::Validate;

#[derive(Validate)]
struct RangeData<'a> {
    #[validate(range(exclusive_min = 1, min = 2))]
    value: &'a str,
}

fn main() {}
