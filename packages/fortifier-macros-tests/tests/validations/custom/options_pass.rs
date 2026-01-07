use fortifier::{Validate, error_code};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Validate)]
struct CustomData<'a> {
    #[validate(custom(function = custom, error = CustomError))]
    zero_options: &'a str,

    #[validate(custom(function = custom, error = CustomError))]
    strip_one_option: Option<&'a str>,
    #[validate(custom(function = custom, error = CustomError))]
    strip_two_options: Option<Option<&'a str>>,
    #[validate(custom(function = custom, error = CustomError))]
    strip_three_options: Option<Option<Option<&'a str>>>,

    #[validate(custom(function = custom_one_option, error = CustomError, options))]
    strip_no_options_from_one: Option<&'a str>,
    #[validate(custom(function = custom_two_options, error = CustomError, options))]
    strip_no_options_from_two: Option<Option<&'a str>>,
    #[validate(custom(function = custom_three_options, error = CustomError, options))]
    strip_no_options_from_three: Option<Option<Option<&'a str>>>,

    #[validate(custom(function = custom_one_option, error = CustomError, options = 1))]
    strip_to_one_option_from_one: Option<&'a str>,
    #[validate(custom(function = custom_one_option, error = CustomError, options = 1))]
    strip_to_one_option_from_two: Option<Option<&'a str>>,
    #[validate(custom(function = custom_one_option, error = CustomError, options = 1))]
    strip_to_one_option_from_three: Option<Option<Option<&'a str>>>,

    #[validate(custom(function = custom_one_option, error = CustomError, options = 2))]
    strip_to_two_options_from_one: Option<&'a str>,
    #[validate(custom(function = custom_two_options, error = CustomError, options = 2))]
    strip_to_two_options_from_two: Option<Option<&'a str>>,
    #[validate(custom(function = custom_two_options, error = CustomError, options = 2))]
    strip_to_two_options_from_three: Option<Option<Option<&'a str>>>,
}

error_code!(CustomErrorCode, CUSTOM_ERROR_CODE, "custom");

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct CustomError {
    code: CustomErrorCode,
}

fn custom(value: &str) -> Result<(), CustomError> {
    if value == "" {
        Ok(())
    } else {
        Err(CustomError {
            code: CustomErrorCode,
        })
    }
}

fn custom_one_option(value: &Option<&str>) -> Result<(), CustomError> {
    if let Some(value) = value
        && *value == ""
    {
        Ok(())
    } else {
        Err(CustomError {
            code: CustomErrorCode,
        })
    }
}

fn custom_two_options(value: &Option<Option<&str>>) -> Result<(), CustomError> {
    if let Some(Some(value)) = value
        && *value == ""
    {
        Ok(())
    } else {
        Err(CustomError {
            code: CustomErrorCode,
        })
    }
}

fn custom_three_options(value: &Option<Option<Option<&str>>>) -> Result<(), CustomError> {
    if let Some(Some(Some(value))) = value
        && *value == ""
    {
        Ok(())
    } else {
        Err(CustomError {
            code: CustomErrorCode,
        })
    }
}

fn main() {
    let data = CustomData {
        zero_options: "",

        strip_one_option: Some(""),
        strip_two_options: Some(Some("")),
        strip_three_options: Some(Some(Some(""))),

        strip_no_options_from_one: Some(""),
        strip_no_options_from_two: Some(Some("")),
        strip_no_options_from_three: Some(Some(Some(""))),

        strip_to_one_option_from_one: Some(""),
        strip_to_one_option_from_two: Some(Some("")),
        strip_to_one_option_from_three: Some(Some(Some(""))),

        strip_to_two_options_from_one: Some(""),
        strip_to_two_options_from_two: Some(Some("")),
        strip_to_two_options_from_three: Some(Some(Some(""))),
    };

    assert_eq!(data.validate_sync(), Ok(()));
}
