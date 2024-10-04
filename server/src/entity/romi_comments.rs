//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "romi_comments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub cid: u32,
    pub aid: u32,
    pub uid: u32,
    pub created: Option<u32>,
    pub ip: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}