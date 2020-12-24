use bytes::buf::ext::BufExt;
use hyper::{Body, Error, Method, Request, Response};
use std::io::Write;

use crate::build_404_response;
use crate::config::Config;
use common::{GetFileContentResponse, GetFilesResponse, Item, ItemType, SaveFileContentRequest};

use std::sync::Arc;
pub async fn handle_api_request(
    req: Request<Body>,
    config: Arc<Config>,
) -> Result<Response<Body>, Error> {
    let path = req.uri().path();
    let directory = &config.file_directory;
    return if path.starts_with("/api/files") {
        handle_files(req, directory)
    } else if path.starts_with("/api/file/") {
        handle_file(req, directory).await
    } else {
        Ok(build_404_response())
    };
}

fn handle_files(req: Request<Body>, directory: &str) -> Result<Response<Body>, Error> {
    if req.method() != &Method::GET {
        return Ok(build_404_response());
    }
    let file = match req.uri().query() {
        Some(q) => {
            let key_idx = q.find("key=").unwrap();
            &q[key_idx + 4..]
        }
        None => "",
    };

    println!("requested: {}", file);
    let files = std::fs::read_dir(directory)
        .unwrap()
        .filter_map(|res| res.map(|e| e.file_name().into_string()).ok())
        .filter_map(|r| r.ok())
        .filter(|f| f.contains(file))
        .map(|f| Item {
            name: f,
            item_type: ItemType::File,
        })
        .collect::<Vec<_>>();

    let body = serde_json::to_string(&GetFilesResponse { files }).unwrap();
    Ok(Response::builder()
        .status(200)
        .body(Body::from(body))
        .unwrap())
}

async fn handle_file(req: Request<Body>, directory: &str) -> Result<Response<Body>, Error> {
    let file_name = &req.uri().path()[10..];
    let path = format!("{}/{}", directory, file_name);
    match req.method() {
        &Method::GET => {
            println!("requested file content: {}", path);
            let content = std::fs::read_to_string(&path);
            let body = match content {
                Ok(c) => serde_json::to_string(&GetFileContentResponse { content: c })
                    .expect("create GetFileContentResponse"),
                Err(_) => {
                    println!("file not found: {}", &path);
                    std::fs::File::create(path).expect("create empty file");
                    serde_json::to_string(&GetFileContentResponse {
                        content: "".to_string(),
                    })
                    .expect("create GetFileContentResponse")
                }
            };
            Ok(Response::builder()
                .status(200)
                .body(Body::from(body))
                .unwrap())
        }

        &Method::POST => {
            let body = hyper::body::aggregate(req).await?;
            let value: SaveFileContentRequest =
                serde_json::from_reader(body.reader()).expect("parse request body");
            println!("write: {}", &value.content);
            std::fs::File::create(&path)
                .expect(&format!("open file {}", &path))
                .write_all(value.content.as_bytes())
                .expect("write contents");
            Ok(Response::builder().status(200).body(Body::empty()).unwrap())
        }

        _ => Ok(build_404_response()),
    }
}
