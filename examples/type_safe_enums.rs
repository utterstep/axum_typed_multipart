use axum::Router;
use axum::{http::StatusCode, routing::post};
use axum_typed_multipart::{TryFromField, TryFromMultipart, TypedMultipart};
use std::net::SocketAddr;

#[derive(Debug, TryFromField)]
pub enum Sex {
    Male,
    Female,
}

#[derive(TryFromMultipart)]
pub struct MultipartData {
    pub name: String,
    pub sex: Sex,
}

async fn test_multipart(multipart: TypedMultipart<MultipartData>) -> StatusCode {
    println!("name = {}, sex = {:?}", multipart.name, multipart.sex);
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, Router::new().route("/", post(test_multipart)).into_make_service())
        .await
        .unwrap();
}
