use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::quests;

#[derive(Debug, Clone, Insertable, Selectable, Queryable)]
#[diesel(table_name = quests)]
pub struct GuildCommanderEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = quests)]
pub struct AddCommanderEntity {
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = quests)]
pub struct EditCommanderEntity {
    pub name: Option<String>,
    pub description: Option<String>,
    pub guild_commander_id: i32,
    pub updated_at: NaiveDateTime
}