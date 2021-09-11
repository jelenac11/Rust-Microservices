use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::diesel::prelude::*;
use crate::model::model::{NewPost, Post};
use crate::schema::posts::dsl::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Create {
    pub title: String,
    pub description: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Update {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Delete {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Publish {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Get {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Post>>")]
pub struct GetPosts;

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Handler<Create> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_post = NewPost {
            title: msg.title,
            description: msg.description,
        };

        diesel::insert_into(posts)
            .values(new_post)
            .get_result::<Post>(&conn)
    }
}

impl Handler<Update> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(posts)
            .filter(id.eq(msg.id))
            .set((title.eq(msg.title), description.eq(msg.description)))
            .get_result::<Post>(&conn)
    }
}

impl Handler<Delete> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(posts)
            .filter(id.eq(msg.id))
            .get_result::<Post>(&conn)
    }
}

impl Handler<Publish> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Publish, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        diesel::update(posts)
            .filter(id.eq(msg.id))
            .set(published.eq(true))
            .get_result::<Post>(&conn)
    }
}

impl Handler<GetPosts> for DbActor {
    type Result = QueryResult<Vec<Post>>;

    fn handle(&mut self, _: GetPosts, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        posts.filter(published.eq(true)).get_results::<Post>(&conn)
    }
}

impl Handler<Get> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        posts.filter(id.eq(msg.id)).get_result::<Post>(&conn)
    }
}
