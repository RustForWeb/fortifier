use fortifier::{UrlError, Validate, ValidationErrors};
use url::{ParseError, Url};

#[derive(Validate)]
struct UrlData<'a> {
    #[validate(url)]
    r#str: &'a str,
    #[validate(url)]
    string: String,
    #[validate(url)]
    url: Url,
}

fn main() {
    let data = UrlData {
        r#str: "http://",
        string: "localhost/test".to_owned(),
        url: Url::parse("http://localhost").expect("valid url"),
    };

    assert_eq!(
        data.validate_sync(),
        Err(ValidationErrors::from_iter([
            UrlDataValidationError::Str(UrlError::from(ParseError::EmptyHost)),
            UrlDataValidationError::String(UrlError::from(ParseError::RelativeUrlWithoutBase))
        ]))
    );
}
