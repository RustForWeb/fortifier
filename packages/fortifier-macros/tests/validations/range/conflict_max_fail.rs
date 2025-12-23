use fortifier::Validate;

#[derive(Validate)]
struct RangeData<'a> {
    #[validate(range(exclusive_max = 1, max = 2))]
    value: &'a str,
}

fn main() {}
