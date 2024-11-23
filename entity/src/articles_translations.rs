//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "articles_translations")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub articles_id: Option<Uuid>,
	pub languages_code: Option<String>,
	pub title: Option<String>,
	pub slug: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub summary: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub content: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::articles::Entity",
		from = "Column::ArticlesId",
		to = "super::articles::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	Articles,
	#[sea_orm(
		belongs_to = "super::languages::Entity",
		from = "Column::LanguagesCode",
		to = "super::languages::Column::Code",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	Languages,
}

impl Related<super::articles::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Articles.def()
	}
}

impl Related<super::languages::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Languages.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
