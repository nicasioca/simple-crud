use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "suppliers")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub supplier_id: i32,
    #[sea_orm(unique)]
    pub supplier_name: String,
    pub fruit_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::fruits::Entity",
        from = "Column::FruitId",
        to = "super::fruits::Column::FruitId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Fruits,
}

impl Related<super::fruits::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fruits.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
