use std::str::FromStr;

use diesel::{AsChangeset, Connection, Identifiable, Insertable, NullableExpressionMethods, OptionalExtension, PgConnection, Queryable, QueryDsl, RunQueryDsl, Selectable};
use diesel::dsl::min;
use diesel::expression_methods::ExpressionMethods;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use uuid::Uuid;

use schema::text_chunk::dsl::*;

use crate::schema::text_chunk;
use crate::schema::text_chunk::text_meta_id;

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

    let result = text_chunk
        .filter(text_meta_id.eq(uuid))
        .filter(
            num.nullable().eq(
                text_chunk
                    .select(min(num))
                    .filter(text_meta_id.eq(uuid))
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
