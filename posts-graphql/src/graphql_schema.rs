use juniper::{RootNode, EmptySubscription, FieldResult, FieldError, graphql_value, GraphQLInputObject};
use crate::{PgPool, repositories::author_repo, repositories::post_repo};
use std::sync::Arc;
use async_trait::async_trait;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use std::collections::HashMap;

pub struct Context {
  pub db: Arc<PgPool>,
  pub loader: PostLoader
}

impl juniper::Context for Context {}

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub author_id: i32,
}

#[juniper::graphql_object(description = "Post that some author created", Context = Context)]
impl Post {
    pub fn id(&self) -> i32 {
        self.id  
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn author_id(&self) -> i32 {
        self.author_id  
    }

    #[graphql(deprecation = "This field is deprecated")]
    pub fn is_published(&self) -> bool {
        true
    }
}

#[derive(Queryable)]
pub struct Author {
    pub id: i32,
    pub name: String
}

#[juniper::graphql_object(description = "Author of posts", Context = Context)]
impl Author {
    pub fn id(&self) -> i32 {
        self.id  
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn posts(&self, ctx: &Context) -> FieldResult<Vec<Post>> {
        let conn = ctx.db.get().expect("Can't get DB connection");
        let author_posts = author_repo::get_author_posts(self.id, &conn);

        Ok(author_posts)
    }
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {

    pub fn api_version() -> &str {
        "1.0"
    }

    #[graphql(description = "List of all authors")]
    async fn authors(ctx: &Context) -> FieldResult<Vec<Author>> {
        let conn = ctx.db.get().expect("Can't get DB connection");
        let authors = author_repo::get_all_authors(&conn);

        Ok(authors)
    }

    #[graphql(description = "Get single author reference by author ID")]
    async fn author(context: &Context, id: i32) -> FieldResult<Author> {
        let conn = context.db.get().expect("Can't get DB connection");
        let author = author_repo::get_author(id, &conn);

        if let Err(_err) = author {
            return Err(FieldError::new(
                "Author Not Found",
                graphql_value!({ "not_found": "author not found" }),
            ));
        } 

        Ok(author.unwrap())
    }

    #[graphql(description = "List of all posts")]
    async fn posts(ctx: &Context) -> FieldResult<Vec<Post>> {
        let conn = ctx.db.get().expect("Can't get DB connection");
        let posts = post_repo::get_all_posts(&conn);

        Ok(posts)
    }

    #[graphql(description = "Get single post reference by post ID")]
    async fn post(context: &Context, id: i32) -> Post {
        context.loader.load(id).await
    }

}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {

    async fn create_author(context: &Context, new: AuthorInput) -> FieldResult<Author> {
        let conn = context.db.get().expect("Can't get DB connection");
        let created_author = author_repo::create_author(new, &conn);

        match created_author {
            Ok(_) => Ok(created_author.unwrap()),
            Err(err) => {
                let msg = err.to_string();
                Err(FieldError::new(
                    "Failed to create new author",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }

    async fn create_post(context: &Context, new: PostInput) -> FieldResult<Post> {
        let conn = context.db.get().expect("Can't get DB connection");
        let created_post = post_repo::create_post(new, &conn);

        match created_post {
            Ok(_) => Ok(created_post.unwrap()),
            Err(err) => {
                let msg = err.to_string();
                Err(FieldError::new(
                    "Failed to create new post",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Author Input")]
pub struct AuthorInput {
    pub name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Post Input")]
pub struct PostInput {
    pub title: String,
    pub description: String,
    pub author_id: i32
}

pub struct PostBatcher {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl BatchFn<i32, Post> for PostBatcher {

    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Post> {
        let mut post_hashmap = HashMap::new();
        self.get_post_by_ids(&mut post_hashmap, keys.to_vec());
        post_hashmap
    }
}

impl PostBatcher {
    pub fn get_post_by_ids(&self, hashmap: &mut HashMap<i32, Post>, ids: Vec<i32>) {
        for id in ids {
            let conn = self.pool.get().expect("Can't get connection from database...");
            let post = post_repo::get_post(id, &conn);
            if let Ok(p) = post {
                hashmap.insert(p.id, p);
            }
        }
      }
}

pub type PostLoader = Loader<i32, Post, PostBatcher>;

pub fn get_loader() -> PostLoader {
    let pool = Arc::new(crate::create_connection_pool());
    Loader::new(PostBatcher { pool })
      .with_yield_count(100)
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}