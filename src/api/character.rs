use crate::database::models::{Character, NewCharacter};
use diesel::prelude::*;

pub fn get_character(
    conn: &mut PgConnection,
    name: &str,
) -> Result<Character, diesel::result::Error> {
    use crate::database::schema::characters::dsl::*;

    characters.filter(character_name.eq(name)).first(conn)
}

pub fn create_chracter(conn: &mut PgConnection, name: &str) {
    use crate::database::schema::characters;

    let new_character = NewCharacter {
        character_name: name,
        user_id: None,
    };

    diesel::insert_into(characters::table)
        .values(&new_character)
        .returning(Character::as_returning())
        .get_result(conn)
        .expect("Error saving new character");
}
