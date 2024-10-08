//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "romi_metas")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub mid: u32,
    pub name: String,
    pub count: Option<String>,
    #[sea_orm(column_name = "isCategory")]
    pub is_category: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
