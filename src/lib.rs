use hyper::{Body, Error, Method, Request, Response, Server};
use std::borrow::Cow;
use std::sync::Arc;

use rust_embed::RustEmbed;

mod api_handler;
use api_handler::handle_api_request;

mod config;
pub use config::Config;

#[derive(RustEmbed)]
#[folder = "static"]
struct Asset;

fn build_404_response() -> Response<Body> {
    return Response::builder().status(404).body(Body::empty()).unwrap();
}

async fn handle_static(req: Request<Body>) -> Result<Response<Body>, Error> {
    let real_path = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => "index.html",
        (&Method::GET, path) => &path[1..],
        _ => {
            return Ok(build_404_response());
        }
    };
    match Asset::get(real_path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            Ok(Response::builder().status(200).body(body).unwrap())
        }
        None => Ok(build_404_response()),
    }
}

pub fn start_server(port: Option<u16>, config: Config) -> (u16, tokio::task::JoinHandle<()>) {
    let port = match port {
        Some(p) => p,
        None => 0,
    };
    let addr = ([127, 0, 0, 1], port).into();
    let server = Server::bind(&addr).serve(MakeService {
        config: Arc::new(config),
    });
    let port = server.local_addr().port();
    println!("Server started on http://127.0.0.1:{}", port);
    (
        port,
        tokio::spawn(async move {
            if let Err(e) = server.await {
                eprintln!("{}", e);
            }
        }),
    )
}

#[tokio::main]
#[allow(unused_must_use)]
async fn main() {
    start_server(Some(8080), Config::default()).1.await;
}

struct Service {
    config: Arc<Config>,
}
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

impl hyper::service::Service<Request<Body>> for Service {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let config = self.config.clone();
        if req.uri().path().starts_with("/api") {
            Box::pin(handle_api_request(req, config))
        } else {
            Box::pin(handle_static(req))
        }
    }
}

struct MakeService {
    config: Arc<Config>,
}

impl<T> hyper::service::Service<T> for MakeService {
    type Response = Service;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: T) -> Self::Future {
        let config = self.config.clone();
        let fut = async move { Ok(Service { config }) };
        Box::pin(fut)
    }
}
