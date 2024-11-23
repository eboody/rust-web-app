//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_webhooks")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub name: String,
	pub method: String,
	pub url: String,
	pub status: String,
	pub data: bool,
	pub actions: String,
	pub collections: String,
	pub headers: Option<Json>,
	pub was_active_before_deprecation: bool,
	pub migrated_flow: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::directus_flows::Entity",
		from = "Column::MigratedFlow",
		to = "super::directus_flows::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	DirectusFlows,
}

impl Related<super::directus_flows::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusFlows.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}