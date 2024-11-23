//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_files")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub storage: String,
	pub filename_disk: Option<String>,
	pub filename_download: String,
	pub title: Option<String>,
	pub r#type: Option<String>,
	pub folder: Option<Uuid>,
	pub uploaded_by: Option<Uuid>,
	pub created_on: DateTimeWithTimeZone,
	pub modified_by: Option<Uuid>,
	pub modified_on: DateTimeWithTimeZone,
	pub charset: Option<String>,
	pub filesize: Option<i64>,
	pub width: Option<i32>,
	pub height: Option<i32>,
	pub duration: Option<i32>,
	pub embed: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub description: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub location: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub tags: Option<String>,
	pub metadata: Option<Json>,
	pub focal_point_x: Option<i32>,
	pub focal_point_y: Option<i32>,
	pub tus_id: Option<String>,
	pub tus_data: Option<Json>,
	pub uploaded_on: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::articles::Entity")]
	Articles,
	#[sea_orm(
		belongs_to = "super::directus_folders::Entity",
		from = "Column::Folder",
		to = "super::directus_folders::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	DirectusFolders,
	#[sea_orm(
		belongs_to = "super::directus_users::Entity",
		from = "Column::ModifiedBy",
		to = "super::directus_users::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	DirectusUsers2,
	#[sea_orm(
		belongs_to = "super::directus_users::Entity",
		from = "Column::UploadedBy",
		to = "super::directus_users::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	DirectusUsers1,
}

impl Related<super::articles::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Articles.def()
	}
}

impl Related<super::directus_folders::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusFolders.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
