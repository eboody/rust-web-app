//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_panels")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub dashboard: Uuid,
	pub name: Option<String>,
	pub icon: Option<String>,
	pub color: Option<String>,
	pub show_header: bool,
	#[sea_orm(column_type = "Text", nullable)]
	pub note: Option<String>,
	pub r#type: String,
	pub position_x: i32,
	pub position_y: i32,
	pub width: i32,
	pub height: i32,
	pub options: Option<Json>,
	pub date_created: Option<DateTimeWithTimeZone>,
	pub user_created: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::directus_dashboards::Entity",
		from = "Column::Dashboard",
		to = "super::directus_dashboards::Column::Id",
		on_update = "NoAction",
		on_delete = "Cascade"
	)]
	DirectusDashboards,
	#[sea_orm(
		belongs_to = "super::directus_users::Entity",
		from = "Column::UserCreated",
		to = "super::directus_users::Column::Id",
		on_update = "NoAction",
		on_delete = "SetNull"
	)]
	DirectusUsers,
}

impl Related<super::directus_dashboards::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusDashboards.def()
	}
}

impl Related<super::directus_users::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::DirectusUsers.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}