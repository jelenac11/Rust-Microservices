#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
use std::pin::Pin;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::PgConnection;
use dotenv::dotenv;
use futures::Stream;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};

use grpc::posts::posts_info_server:: {
    PostsInfo, PostsInfoServer,
};
use grpc::posts::{
    Post, PostRequest, PostResponse, PostsResponse,
};

mod db;
mod repository;
mod model;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "[::1]:50051".parse().unwrap();

    let pool = db::create_connection_pool();

    let posts_info = PostInfoService { pool };
    let pvc = PostsInfoServer::new(posts_info);

    Server::builder().add_service(pvc).serve(addr).await?;

    Ok(())
}

struct PostInfoService {
    pool: db::PgPool,
}

#[tonic::async_trait]
impl PostsInfo for PostInfoService {
    type GetPostsStreamStream =
        Pin<Box<dyn Stream<Item = Result<PostResponse, Status>> + Send + Sync + 'static>>;

    async fn get_posts(
        &self,
        _: Request<()>,
    ) -> Result<Response<PostsResponse>, Status> {
        let posts_titles = repository::get_titles(&get_connection(&self.pool))
            .expect("Can't get titles of the posts");

        let reply = PostsResponse {
            post: posts_titles,
        };

        Ok(Response::new(reply))
    }

    async fn get_posts_stream(
        &self,
        _: Request<()>,
    ) -> Result<Response<Self::GetPostsStreamStream>, Status> {
        let (tx, rx) = mpsc::channel(4);

        let posts: Vec<Post> = repository::get_all(&get_connection(&self.pool))
            .expect("Can't load posts")
            .into_iter()
            .map(|p| {
                Post {
                    id: p.id as u64,
                    title: p.title,
                    text: p.text,
                }
                .into()
            })
            .collect();

        tokio::spawn(async move {
            let mut stream = tokio_stream::iter(&posts);

            while let Some(post) = stream.next().await {
                tx.send(Ok(PostResponse {
                    post: Some(post.clone()),
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn get_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let id = request.into_inner().id;

        let post = repository::get_by_id(id, &get_connection(&self.pool));

        match post {
            Ok(p) => {
                let post = Post {
                    id: p.id as u64,
                    title: p.title,
                    text: p.text
                }
                .into();

                let reply = PostResponse {
                    post: Some(post),
                };

                Ok(Response::new(reply))
            }
            Err(e) => {
                match e {
                    Error::NotFound => Err(Status::not_found(format!(
                        "Post with id {} not found",
                        &id
                    ))),
                    _ => Err(Status::unknown(format!(
                        "There was an error while getting a post {}: {}",
                        &id, e
                    ))),
                }
            }
        }
    }
}

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

fn get_connection(pool: &db::PgPool) -> Conn {
    pool.get().expect("Can't get DB connection")
}
