use fortifier::Validate;

#[derive(Validate)]
struct RangeData<'a> {
    #[validate(range(unknown = 2))]
    value: &'a str,
}

fn main() {}
