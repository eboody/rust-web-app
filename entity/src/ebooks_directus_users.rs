//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ebooks_directus_users")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub ebooks_id: Option<Uuid>,
	pub directus_users_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::directus_users::Entity",
		from = "Column::DirectusUsersId",
		to = "super::directus_users::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	DirectusUsers,
	#[sea_orm(
		belongs_to = "super::ebooks::Entity",
		from = "Column::EbooksId",
		to = "super::ebooks::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	Ebooks,
}

impl Related<super::directus_users::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusUsers.def()
	}
}

impl Related<super::ebooks::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Ebooks.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}