-- Your SQL goes here
CREATE TABLE text_chunk (
    id uuid not null,
    text_meta_id uuid not null,
    num int4 not null,
    content text not null,
    primary key (id)
)