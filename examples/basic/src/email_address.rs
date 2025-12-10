use fortifier::Validate;

#[derive(Validate)]
pub enum ChangeEmailAddressRelation {
    Create {
        #[validate(email)]
        email_address: String,
    },
    Update {
        id: String,

        #[validate(email)]
        email_address: String,
    },
    Delete {
        id: String,
    },
}
