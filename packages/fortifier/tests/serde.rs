#![cfg(feature = "serde")]

use fortifier::{EmailError, LengthError, RegexError, UrlError, ValidationErrors};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use url::ParseError;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "camelCase")]
enum TestError {
    Email(EmailError),
    Length(LengthError<usize>),
    Regex(RegexError),
    Url(UrlError),
}

fn setup() -> (ValidationErrors<TestError>, Value) {
    (
        ValidationErrors::from_iter([
            TestError::Email(EmailError::from(email_address::Error::MissingSeparator)),
            TestError::Length(LengthError::Equal {
                equal: 1,
                length: 2,
                message: "length 2 is not equal to required length 1".to_owned(),
            }),
            TestError::Regex(RegexError::default()),
            TestError::Url(UrlError::from(ParseError::EmptyHost)),
        ]),
        #[cfg(not(feature = "message"))]
        json!([
            {
                "code": "email",
                "subcode": "missingSeparator",
            },
            {
                "code": "length",
                "subcode": "equal",
                "equal": 1,
                "length": 2,
            },
            {
                "code": "regex",
            },
            {
                "code": "url",
                "subcode": "emptyHost",
            }
        ]),
        #[cfg(feature = "message")]
        json!([
            {
                "code": "email",
                "subcode": "missingSeparator",
                "message": "",
            },
            {
                "code": "length",
                "subcode": "equal",
                "equal": 1,
                "length": 2,
                "message": "length 2 is not equal to required length 1",
            },
            {
                "code": "regex",
                "message": "value does not match regular expression",
            },
            {
                "code": "url",
                "subcode": "emptyHost",
                "message": "empty host",
            }
        ]),
    )
}

#[test]
fn serialize() {
    let (deserialized, serialized) = setup();

    assert_eq!(serde_json::to_value(&deserialized).unwrap(), serialized);
}

#[test]
fn deserialize() {
    let (deserialized, serialized) = setup();

    assert_eq!(
        serde_json::from_value::<ValidationErrors<TestError>>(serialized).unwrap(),
        deserialized,
    );
}
