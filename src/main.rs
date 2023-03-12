use std::str::FromStr;

use diesel::alias;
use diesel::dsl::min;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use uuid::Uuid;

use crate::schema::text_chunk::{num, text_meta_id};
use crate::schema::text_chunk;

mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Queryable, Identifiable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = text_chunk)]
pub struct DTO {
    pub id: Uuid,
    pub text_meta_id: Uuid,
    pub num: i32,
    pub content: String,
}

fn main() {
    dotenv().ok();

    let connection = &mut PgConnection::establish(std::env::var("DATABASE_URL").unwrap().as_str()).unwrap();

    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let uuid = Uuid::from_str("0e411b6f-be41-4260-b577-ea93c8ab7634").unwrap();

    let (chunk1, chunk2) = alias!(text_chunk as chunk1, text_chunk as chunk2);

    let result = chunk1
        .filter(chunk1.field(text_meta_id).eq(uuid))
        .filter(
            chunk1.field(num)
                .nullable()
                .eq(
                    chunk2
                        .select(min(chunk2.field(num)))
                        .filter(chunk2.field(text_meta_id).eq(uuid))
                        .single_value()
                )
        )
        .first::<DTO>(connection)
        .optional()
        .unwrap();

    if let Some(data) = result {
        println!("{}", data.content)
    } else {
        println!("Not found")
    }
}
