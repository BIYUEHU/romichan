//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "romi_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid: u32,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub salt: String,
    #[sea_orm(unique)]
    pub email: String,
    pub created: Option<u32>,
    #[sea_orm(column_name = "lastLogin")]
    pub last_login: Option<u32>,
    #[sea_orm(column_name = "isAdmin")]
    pub is_admin: Option<String>,
    #[sea_orm(column_name = "isDeleted")]
    pub is_deleted: Option<String>,
    pub url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}