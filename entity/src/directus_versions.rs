//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_versions")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub key: String,
	pub name: Option<String>,
	pub collection: String,
	pub item: String,
	pub hash: Option<String>,
	pub date_created: Option<DateTimeWithTimeZone>,
	pub date_updated: Option<DateTimeWithTimeZone>,
	pub user_created: Option<Uuid>,
	pub user_updated: Option<Uuid>,
	pub delta: Option<Json>,
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
	#[sea_orm(has_many = "super::directus_revisions::Entity")]
	DirectusRevisions,
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

impl Related<super::directus_revisions::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusRevisions.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
