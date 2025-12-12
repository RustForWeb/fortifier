#[cfg(feature = "fortifier")]
use fortifier::Validate;
#[cfg(all(feature = "validator", not(feature = "fortifier")))]
use validator::Validate;

#[derive(Validate)]
pub struct Struct1 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct2 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct3 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct4 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct5 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct6 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct7 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct8 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct9 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}

#[derive(Validate)]
pub struct Struct10 {
    #[validate(length(min = 1, max = 256))]
    pub field1: String,
    #[validate(length(min = 1, max = 256))]
    pub field2: String,
    #[validate(length(min = 1, max = 256))]
    pub field3: String,
    #[validate(length(min = 1, max = 256))]
    pub field4: String,
    #[validate(length(min = 1, max = 256))]
    pub field5: String,
    #[validate(length(min = 1, max = 256))]
    pub field6: String,
    #[validate(length(min = 1, max = 256))]
    pub field7: String,
    #[validate(length(min = 1, max = 256))]
    pub field8: String,
    #[validate(length(min = 1, max = 256))]
    pub field9: String,
    #[validate(length(min = 1, max = 256))]
    pub field10: String,
}
