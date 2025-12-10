pub mod user {
    use serde::Serialize;
    use utoipa::ToSchema;
    use uuid::Uuid;

    #[derive(Serialize, ToSchema)]
    pub struct Model {
        pub id: Uuid,
        pub email_address: String,
        pub name: String,
    }
}
