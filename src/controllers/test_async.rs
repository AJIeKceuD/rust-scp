use std::sync::Arc;
use std::str;
use hyper::{Request, Body, Response, HeaderMap, StatusCode};
use hyper::body::Bytes;
use hyper::http::request::Parts;
use log::{error, warn, info, debug, trace};

// use serde_json::Result as serde_result;

// #[path="./../services/logs/db_log_object.rs"]
// mod db_log_object;
// use db_log_object::DBLogObject as DBLogObject;
use crate::ServerContext;
use crate::router::model::log::{LogModel, RequestId};
use crate::router::RequestContext;

// use web_controller::WebController;
// use web_controller_derive::WebController;

// #[path="../helpers.rs"]
// mod helpers; // why not work? How make it work? Or use like this is bad idea?

pub struct TestAsyncController {
    pub tmp_str: String,
    pub request_context: RequestContext,
    pub server_context: Arc<ServerContext>,
}

impl TestAsyncController {
    // Is it possible use new() from derive directly?
    pub async fn new(server_context: Arc<ServerContext>, request_context: RequestContext) -> Result<Self, hyper::Error> {
        // TestController::new(); // fall in recursion and panic
        // &self.new_init();
        // let (req_parts, req_body) = request.into_parts();
        //
        // let full_body = hyper::body::to_bytes(req_body).await?;

        Ok(Self {
            tmp_str: String::from("fd435"),
            request_context: request_context,
            server_context: server_context,
        })
    }

    pub async fn index(&self) -> Result<Response<Body>, hyper::Error> {
        // let req = &self.request;
        info!("TEST req_h {:?}", &self.request_context.request_parts);
        info!("TEST full_body {:?}", &self.request_context.full_body);
        let server_context = &self.server_context;

        // let mut response = Response::new(Body::empty());
        //
        // *response.body_mut() = String::from("asds3432432d").into();
        // *response
        //     .header("Foo", "Bar")
        //     .status(StatusCode::NOT_FOUND);

        // TODO remake unwrap
        let response = Response::builder()
            .header("Foo", "Bar")
            .header("Content-Type", "application/json")
            .status(StatusCode::OK)
            .body(String::from("asds3432432d").into())
            .unwrap();

        Ok(response)
    }

    // pub
}
