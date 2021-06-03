use std::sync::Arc;
use std::str;
use futures::executor::block_on;
use hyper::{Request, Body, Response, HeaderMap};
use hyper::body::Bytes;
use log::{error, warn, info, debug, trace};

use crate::ServerContext;
// use serde_json::Result as serde_result;

#[path="./../services/logs/db_log_object.rs"]
mod db_log_object;
use db_log_object::DBLogObject as DBLogObject;

// use web_controller::WebController;
// use web_controller_derive::WebController;

// #[path="../helpers.rs"]
// mod helpers; // why not work? How make it work? Or use like this is bad idea?

pub struct TestAsyncController {
    pub tmp_str: String,
    pub request: Request<Body>,
    pub server_context: Arc<ServerContext>,
}

impl TestAsyncController {
    // Is it possible use new() from derive directly?
    pub fn new(&mut self) {
        // TestController::new(); // fall in recursion and panic
        // &self.new_init();
    }

    pub async fn default(&mut self) -> Result<Response<Body>, hyper::Error> {
        // let req = &self.request;
        info!("TEST req_h {:?}", &self.request.headers());
        info!("TEST req_body {:?}", &self.request.body());
        let server_context = &self.server_context;

        let mut response = Response::new(Body::empty());
        let stage = String::from("init");
        // let (req_parts, req_body) = &self.request.into_parts();
        let req_body = &self.request.into_body();
        info!("TEST req_body {:?}", req_body);
        // let req_body = **req_body;
        let req_headers = &self.request.headers();
        // let req_headers = req_parts.headers.clone();

        /** Next block create body_str variable, but easier way is using format!(). Leave this for memory :)
         Plus we anyway need async part to get whole body, not "Stream". */
        // Await the full body to be concatenated into a single `Bytes`...
        let full_body: Bytes =
            match hyper::body::to_bytes(req_body).await {
                Ok(full_body) => {
                    full_body
                },
                Err(e) => {
                    hyper::body::Bytes::new()
                }
            }
        ;
        info!("TEST full_body {:?}", full_body);
        // Iterate the full body and collect into a new Vec.
        let body_vec = full_body.iter()
            .cloned()
            .collect::<Vec<u8>>();
        info!("TEST body_vec {:?}", body_vec);
        let body_str = str::from_utf8(&body_vec).unwrap();
        info!("TEST body_str {}", body_str);
        // let body_serialized = serde_json::to_string(body_str).unwrap();
        // info!("TEST body_serialized {}", body_serialized);

        let log = DBLogObject {
            request_id: 1234,
            payment_id: 123456,
            stage: stage.into(),
            log_type: "".into(),
            microtime_bgn: 0,
            microtime_end: 0,
            result: 0,
            http_code: 200,
            send_data: body_str.into(), //format!("{:?}", full_body),
            send_headers: format!("{:?}", req_headers),
            receive_data: "".into(),
            receive_headers: "".into(),
        };
        log_db!(log, server_context.db_pool);
        // let serialized = serde_json::to_string(&log).unwrap();
        // info!("{}", serialized);

        *response.body_mut() = String::from("asds3432432d").into();

        Ok(response)
    }
}
