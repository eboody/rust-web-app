//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_comments")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub collection: String,
	pub item: String,
	#[sea_orm(column_type = "Text")]
	pub comment: String,
	pub date_created: Option<DateTimeWithTimeZone>,
	pub date_updated: Option<DateTimeWithTimeZone>,
	pub user_created: Option<Uuid>,
	pub user_updated: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::directus_collections::Entity",
		from = "Column::Collection",
		to = "super::directus_collections::Column::Collection",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	DirectusCollections,
	#[sea_orm(
		belongs_to = "super::directus_users::Entity",
		from = "Column::UserCreated",
		to = "super::directus_users::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	DirectusUsers2,
	#[sea_orm(
		belongs_to = "super::directus_users::Entity",
		from = "Column::UserUpdated",
		to = "super::directus_users::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	DirectusUsers1,
}

impl Related<super::directus_collections::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusCollections.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}