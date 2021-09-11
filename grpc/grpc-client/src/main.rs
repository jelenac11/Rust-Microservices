use tonic::Request;

use grpc::posts::posts_info_client::PostsInfoClient;
use grpc::posts::{Post, PostRequest, PostResponse, PostsResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PostsInfoClient::connect("http://[::1]:50051").await?;

    let request = Request::new(PostRequest { id: 1 });

    let response = client.get_post(request.into_inner()).await?;
    println!("RESPONSE={:?}", response);

    let response = client.get_posts(Request::new(()).into_inner()).await?;
    let message = response
        .into_inner()
        .post
        .into_iter()
        .map(|p| format!("Post: {}", p))
        .collect::<Vec<String>>()
        .join("; ");
    println!("POSTS -> {:?}", message);

    let response = client
        .get_posts_stream(Request::new(()).into_inner())
        .await?;
    let mut posts_stream = response.into_inner();
    while let Some(response) = posts_stream.message().await.expect("Can't get a post") {
        println!("NOTE = {:?}", response);
    }

    Ok(())
}
