//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ebooks_translations")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub ebooks_id: Option<Uuid>,
	pub languages_code: Option<String>,
	pub cover_image: Option<Uuid>,
	#[sea_orm(column_type = "Text", nullable)]
	pub content: Option<String>,
	pub title: Option<String>,
	pub slug: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub summary: Option<String>,
	pub file: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::directus_files::Entity",
		from = "Column::CoverImage",
		to = "super::directus_files::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	DirectusFiles2,
	#[sea_orm(
		belongs_to = "super::directus_files::Entity",
		from = "Column::File",
		to = "super::directus_files::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	DirectusFiles1,
	#[sea_orm(
		belongs_to = "super::ebooks::Entity",
		from = "Column::EbooksId",
		to = "super::ebooks::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	Ebooks,
	#[sea_orm(
		belongs_to = "super::languages::Entity",
		from = "Column::LanguagesCode",
		to = "super::languages::Column::Code",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	Languages,
}

impl Related<super::ebooks::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Ebooks.def()
	}
}

impl Related<super::languages::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Languages.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
