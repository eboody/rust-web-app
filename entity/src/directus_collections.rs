//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_collections")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub collection: String,
	pub icon: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub note: Option<String>,
	pub display_template: Option<String>,
	pub hidden: bool,
	pub singleton: bool,
	pub translations: Option<Json>,
	pub archive_field: Option<String>,
	pub archive_app_filter: bool,
	pub archive_value: Option<String>,
	pub unarchive_value: Option<String>,
	pub sort_field: Option<String>,
	pub accountability: Option<String>,
	pub color: Option<String>,
	pub item_duplication_fields: Option<Json>,
	pub sort: Option<i32>,
	pub group: Option<String>,
	pub collapse: String,
	pub preview_url: Option<String>,
	pub versioning: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "Entity",
		from = "Column::Group",
		to = "Column::Collection",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	SelfRef,
	#[sea_orm(has_many = "super::directus_comments::Entity")]
	DirectusComments,
	#[sea_orm(has_many = "super::directus_shares::Entity")]
	DirectusShares,
	#[sea_orm(has_many = "super::directus_versions::Entity")]
	DirectusVersions,
}

impl Related<super::directus_comments::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusComments.def()
	}
}

impl Related<super::directus_shares::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusShares.def()
	}
}

impl Related<super::directus_versions::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusVersions.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
