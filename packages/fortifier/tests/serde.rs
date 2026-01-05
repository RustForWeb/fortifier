#![cfg(feature = "serde")]

use cfg_if::cfg_if;
use fortifier::{
    EmailAddressError, LengthError, LengthErrorCode, RegexError, UrlError, ValidationErrors,
};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use url::ParseError;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "camelCase")]
enum TestError {
    EmailAddress(EmailAddressError),
    Length(LengthError<usize>),
    Regex(RegexError),
    Url(UrlError),
}

fn setup() -> (ValidationErrors<TestError>, Value) {
    let errors = ValidationErrors::from_iter([
        TestError::EmailAddress(EmailAddressError::from(
            email_address::Error::MissingSeparator,
        )),
        TestError::Length(LengthError::Equal {
            equal: 1,
            value: 2,
            code: LengthErrorCode,
            #[cfg(feature = "message")]
            message: "length 2 is not equal to required length 1".to_owned(),
        }),
        TestError::Regex(RegexError::default()),
        TestError::Url(UrlError::from(ParseError::EmptyHost)),
    ]);

    cfg_if! {
        if #[cfg(feature = "message")] {
             (
                errors,
                json!([
                    {
                        "code": "emailAddress",
                        "subcode": "missingSeparator",
                        "message": "",
                    },
                    {
                        "code": "length",
                        "subcode": "equal",
                        "equal": 1,
                        "value": 2,
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
        } else {
            (
                errors,
                json!([
                    {
                        "code": "emailAddress",
                        "subcode": "missingSeparator",
                    },
                    {
                        "code": "length",
                        "subcode": "equal",
                        "equal": 1,
                        "value": 2,
                    },
                    {
                        "code": "regex",
                    },
                    {
                        "code": "url",
                        "subcode": "emptyHost",
                    }
                ]),
            )
        }
    }
}

#[test]
fn serialize() {
    let (deserialized, serialized) = setup();

    assert_eq!(
        serde_json::to_value(&deserialized).expect("serializable value"),
        serialized
    );
}

#[test]
fn deserialize() {
    let (deserialized, serialized) = setup();

    assert_eq!(
        serde_json::from_value::<ValidationErrors<TestError>>(serialized)
            .expect("deserializable value"),
        deserialized,
    );
}
