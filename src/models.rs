use diesel::prelude::*;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Post {
    pub title: String,
    pub author: String,
    pub url_cover: String,
    pub word_count: i32,
    pub id: i32,
    pub language: String,
}
