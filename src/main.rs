use diesel::{AsChangeset, BoolExpressionMethods, Connection, Identifiable, Insertable, NullableExpressionMethods, OptionalExtension, PgConnection, Queryable, QueryDsl, RunQueryDsl, Selectable};
use diesel::dsl::{min};
use diesel::expression_methods::ExpressionMethods;
use dotenv::dotenv;
use uuid::Uuid;

use schema::text_chunk::dsl::*;

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

    let connection = &mut PgConnection::establish(std::env::var("DATABASE_URL").unwrap().as_str()).unwrap();

    let result = text_chunk
                .filter(
            text_meta_id
                .eq(text_meta_id)
                .and(
                    num.nullable().eq(
                        text_chunk::table
                            .select(min(num))
                            .filter(text_meta_id.eq(text_meta_id))
                            .single_value()
                    )
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
