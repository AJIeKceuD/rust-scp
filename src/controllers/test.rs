use std::sync::Arc;
use std::str;
use futures::executor::block_on;
use hyper::{Request, Body, Response};
use hyper::body::Bytes;
use log::{error, warn, info, debug, trace};

use crate::ServerContext;
// use serde_json::Result as serde_result;

#[path="./../services/logs/db_log_object.rs"]
mod db_log_object;
use db_log_object::DBLogObject as DBLogObject;

use web_controller::WebController;
use web_controller_derive::WebController;

// #[path="../helpers.rs"]
// mod helpers; // why not work? How make it work? Or use like this is bad idea?

#[derive(WebController)]
pub struct TestController {}

impl TestController {
    // Is it possible use new() from derive directly?
    pub fn new() {
        // TestController::new(); // fall in recursion and panic
        TestController::new_log();
    }

    pub fn default(req: Request<Body>, server_context: Arc<ServerContext>) -> Result<Response<Body>, hyper::Error> {
        let mut response = Response::new(Body::empty());
        let stage = String::from("init");
        let (req_parts, req_body) = req.into_parts();
        // let req_body = req.into_body();
        // let req_headers = req.headers();
        // let req_headers = req_parts.headers.clone();

        /** Next block create body_str variable, but easier way is using format!(). Leave this for memory :)
         Plus we anyway need async part to get whole body, not "Stream". */
        // Await the full body to be concatenated into a single `Bytes`...
        let full_body: Bytes = block_on(async {
            match hyper::body::to_bytes(req_body).await {
                Ok(full_body) => {
                    full_body
                },
                Err(e) => {
                    hyper::body::Bytes::new()
                }
            }
        });
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
            send_headers: format!("{:?}", req_parts),
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
