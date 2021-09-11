#[macro_use]
extern crate serde_derive;
use bytes::Bytes;
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use route_recognizer::Params;
use router::Router;
use std::fmt;
use std::sync::{Arc, RwLock};

mod handlers;
mod router;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    description: String,
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}, Description: {}",
            self.title, self.description
        )
    }
}

#[tokio::main]
async fn main() {
    let mut router: Router = Router::new();
    router.get("/posts/:id", Box::new(handlers::get_post_by_id));
    router.post("/posts", Box::new(handlers::create_post));
    router.delete("/posts/:id", Box::new(handlers::delete_post));
    router.put("/posts/:id", Box::new(handlers::update_post));

    let shared_router = Arc::new(router);
    let post_db = Arc::new(RwLock::new(Vec::new()));
    let addr = "0.0.0.0:8080".parse().unwrap();
    let server = Server::bind(&addr).serve(make_service_fn(move |_| {
        let router_capture = shared_router.clone();
        let post_db = post_db.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, post_db.clone())
            }))
        }
    }));
    println!("Listening on http://{}", addr);
    let _ = server.await;
}

async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    db: Arc<RwLock<Vec<Post>>>,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(Context::new(req, found_handler.params), db)
        .await;
    Ok(resp)
}

#[derive(Debug)]
pub struct Context {
    pub req: Request<Body>,
    pub params: Params,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(req: Request<Body>, params: Params) -> Context {
        Context {
            req,
            params,
            body_bytes: None,
        }
    }

    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes not set")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}
