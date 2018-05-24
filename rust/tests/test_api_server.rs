extern crate fatcat;
extern crate fatcat_api;
extern crate iron;
extern crate iron_test;

use iron::{status, Headers};
use iron::mime::Mime;
use iron::headers::ContentType;
use iron_test::{request, response};

#[test]
fn test_basics() {
    let server = fatcat::server().unwrap();
    let router = fatcat_api::router(server);

    let response = request::get(
        "http://localhost:9411/v0/creator/f1f046a3-45c9-4b99-adce-000000000001",
        Headers::new(),
        &router,
    ).unwrap();
    assert_eq!(response.status, Some(status::Ok));
    let body = response::extract_body_to_string(response);
    assert!(body.contains("Grace Hopper"));

    let response = request::get(
        "http://localhost:9411/v0/creator/f1f046a3-45c9-4b99-adce-999999999999",
        Headers::new(),
        &router,
    ).unwrap();
    assert_eq!(response.status, Some(status::NotFound));
}

#[test]
fn test_lookups() {
    let server = fatcat::server().unwrap();
    let router = fatcat_api::router(server);

    let response = request::get(
        "http://localhost:9411/v0/container/lookup?issn=1234-5678",
        Headers::new(),
        &router,
    ).unwrap();
    assert_eq!(response.status, Some(status::Ok));
    let body = response::extract_body_to_string(response);
    assert!(body.contains("Journal of Trivial Results"));

    let response = request::get(
        "http://localhost:9411/v0/creator/lookup?orcid=0000-0003-2088-7465",
        Headers::new(),
        &router,
    ).unwrap();
    assert_eq!(response.status, Some(status::Ok));
    let body = response::extract_body_to_string(response);
    assert!(body.contains("Christine Moran"));
}

#[test]
fn test_post_container() {
    let server = fatcat::server().unwrap();
    let router = fatcat_api::router(server);
    let mut headers = Headers::new();
    let mime: Mime = "application/json".parse().unwrap();
    headers.set(ContentType(mime));


    let response = request::post(
        "http://localhost:9411/v0/container",
        headers,
        r#"{"name": "test journal"}"#,
        &router,
    ).unwrap();
    assert_eq!(response.status, Some(status::Created));
    let body = response::extract_body_to_string(response);
    println!("{}", body);
    //assert!(body.contains("test journal"));
}