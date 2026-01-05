use sea_orm::entity::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "username")]
    pub username: String,
    #[sea_orm(column_name = "email")]
    pub email: String,
    #[sea_orm(column_name = "password")]
    pub password: String,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "points")]
    pub points: i32,
    #[sea_orm(column_name = "avatar")]
    pub avatar: Option<String>,
}
