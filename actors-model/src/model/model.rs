use crate::actix::Addr;
use crate::actors::count::CountActor;
use crate::actors::db::DbActor;
use crate::schema::posts;

#[derive(Clone)]
pub struct AppState {
    pub db: Addr<DbActor>,
    pub count: Addr<CountActor>,
}

#[derive(Debug, Clone, Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub published: bool,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub description: String,
}
