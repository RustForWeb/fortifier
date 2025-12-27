use fortifier::Validate;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Validate)]
struct CreateUser {
    #[validate(length(min = 1, max = 256))]
    name: String,

    email_addresses: Vec<CreateEmailAddress>,
}

#[derive(Deserialize, Serialize, Validate)]
struct CreateEmailAddress {
    #[validate(email_address)]
    email_address: String,
}

fn main() {
    let data = CreateUser {
        name: "".to_owned(),
        email_addresses: vec![CreateEmailAddress {
            email_address: "invalid".to_owned(),
        }],
    };

    assert_eq!(
        serde_json::to_value(data.validate_sync().expect_err("validation error"))
            .expect("serializable value"),
        json!([
            {
                "path": "name",
                "code": "length",
                "subcode": "min",
                "min": 1,
                "value": 0
            },
            {
                "path": "emailAddresses",
                "code": "nested",
                "errors": [
                    {
                        "index": 0,
                        "path": "emailAddress",
                        "code": "emailAddress",
                        "subcode": "missingSeparator"
                    }
                ]
            }
        ]),
    );
}
