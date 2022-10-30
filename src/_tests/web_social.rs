use std::net::{SocketAddr, TcpListener};

use axum::{
    body::Body,
    http::{self, Request},
};
use axum::http::StatusCode;
use serde_json::Value;

use crate::web::app;

//todo should auth with test user
const AUTH_TOKEN: &str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjExLCJjb21wYW55IjoidG9tb3J1IiwiZXhwIjoyMDAwMDAwMDAwfQ.E0_Vc2GDSILvzJwMOJqhnGoSAGjn1UokOrqucONGgcg";

#[sqlx_database_tester::test(pool(variable = "db"))]
async fn when_create_social_then_find_by_id_ok() {
    //given
    let listener = TcpListener::bind(&SocketAddr::from(([127, 0, 0, 1], 3000))).unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app(db).into_make_service())
            .await
            .unwrap();
    });
    let client = hyper::Client::new();

    //when
    let response = client
        .request(
            Request::builder()
                .method(http::Method::POST)
                .uri(format!("http://{}/api/v1/socials", addr))
                .header(http::header::AUTHORIZATION, AUTH_TOKEN)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    "{\"social_type\": \"telegram\"}",
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!("telegram", body.get("social_type").unwrap());
    assert_eq!(1, body.get("id").unwrap().as_i64().unwrap());

    //then
    let response = client
        .request(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("http://{}/api/v1/socials/1", addr))
                .header(http::header::AUTHORIZATION, AUTH_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!("telegram", body.get("social_type").unwrap());
    assert_eq!(1, body.get("id").unwrap().as_i64().unwrap());
}

#[sqlx_database_tester::test(pool(variable = "db"))]
async fn given_unknown_id_when_find_by_id_then_not_found() {
    //given
    let listener = TcpListener::bind(&SocketAddr::from(([127, 0, 0, 1], 3000))).unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app(db).into_make_service())
            .await
            .unwrap();
    });
    let client = hyper::Client::new();

    //when
    let response = client
        .request(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("http://{}/api/v1/socials/10", addr))
                .header(http::header::AUTHORIZATION, AUTH_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    //then
    assert_eq!(StatusCode::NOT_FOUND, response.status());
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!("Entity not found - social[10]", body.get("error_message").unwrap());
}

#[sqlx_database_tester::test(pool(variable = "db"))]
async fn given_no_auth_header_when_find_by_id_then_auth_error() {
    //given
    let listener = TcpListener::bind(&SocketAddr::from(([127, 0, 0, 1], 3000))).unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app(db).into_make_service())
            .await
            .unwrap();
    });
    let client = hyper::Client::new();

    //when
    let response = client
        .request(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("http://{}/api/v1/socials/10", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    //then
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!("invalid auth token", body.get("error_message").unwrap());
}

#[sqlx_database_tester::test(pool(variable = "db"))]
async fn given_no_auth_header_when_create_then_auth_error() {
    //given
    let listener = TcpListener::bind(&SocketAddr::from(([127, 0, 0, 1], 3000))).unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app(db).into_make_service())
            .await
            .unwrap();
    });
    let client = hyper::Client::new();

    //when
    let response = client
        .request(
            Request::builder()
                .method(http::Method::POST)
                .uri(format!("http://{}/api/v1/socials", addr))
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    "{\"social_type\": \"telegram\"}",
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    //then
    assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!("invalid auth token", body.get("error_message").unwrap());
}
