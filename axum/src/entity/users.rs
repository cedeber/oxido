//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub userid: i32,
    pub created: String,
    pub active: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
