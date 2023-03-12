use diesel::{AsChangeset, Connection, Identifiable, Insertable, OptionalExtension, PgConnection, Queryable, QueryDsl, Selectable, RunQueryDsl, BoolExpressionMethods};
use diesel::dsl::min;
use dotenv::dotenv;
use uuid::Uuid;

use crate::schema::text_chunk;
use crate::schema::text_chunk::text_meta_id;

mod schema;

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

    let mut connection = PgConnection::establish(std::env::var("DATABASE_URL").unwrap().as_str());

    let result = text_chunk::table
        .filter(
            text_meta_id
                .eq(text_meta_id)
                .and(
                    text_chunk::num.eq(
                        text_chunk::table
                            .select(min(text_chunk::num))
                            .filter(text_meta_id.eq(text_meta_id))
                            .single_value()
                    )
                )
        )
        .first::<DTO>(&mut connection)
        .optional()
        .unwrap();

    println!("{}", result.unwrap().content)
}
