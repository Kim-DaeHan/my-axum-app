use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

// <'a> 은 라이프타임 매개변수를 나타냄(a라는 라이프타임이 있다)
#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
pub struct PostData {
    pub id: Option<String>,
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
