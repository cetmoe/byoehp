use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Character {
    pub id: i32,
    pub character_name: String,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}
