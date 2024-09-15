use super::schema::characters;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Character {
    pub id: i32,
    pub character_name: String,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "characters"]
pub struct NewCharacter<'a> {
    pub character_name: &'a str,
    pub user_id: Option<i32>,
}
