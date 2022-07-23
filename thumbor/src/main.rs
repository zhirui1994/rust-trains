mod pb;

use std::sync::Arc;

use bytes::Bytes;
use pb::*;
use axum::{extract::Path, http::StatusCode, routing::get, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use lru::LruCache;
use tokio::sync::Mutex;
use tower::{ServiceBuilder};
use tower_http::{
    add_extension::AddExtensionLayer
};
use tracing::info;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));

    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
            .layer(AddExtensionLayer::new(cache))
            .into_inner()
        );
    
    let addr = "127.0.0.1:3000".parse().unwrap();

    tracing::debug!("listening on {}", addr);

    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
