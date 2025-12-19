pub mod email_address {
    use serde::Serialize;
    use utoipa::ToSchema;
    use uuid::Uuid;

    #[derive(Serialize, ToSchema)]
    pub struct Model {
        pub id: Uuid,
        pub email_addres: String,
        pub label: String,
    }
}
