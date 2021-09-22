use std::sync::Arc;
use std::collections::HashMap;
use futures::io::ErrorKind;

use hyper::{Request, Body, Response, HeaderMap, StatusCode};
use hyper::body::Bytes;
use hyper::http::request::Parts;

use serde::{Deserialize, Serialize};
// use serde_json::json;

use log::{error, warn, info, debug, trace};

// use serde_json::Result as serde_result;

// #[path="./../services/logs/db_log_object.rs"]
// mod db_log_object;
// use db_log_object::DBLogObject as DBLogObject;
use crate::ServerContext;
use crate::router::model::log::{LogModel, RequestId};
use crate::router::{RequestContext, ControllerResponse};

#[path="../services/mapper/codes.rs"]
mod codes;
use codes::InnerCode;
use codes::InnerResult;
use codes::InnerResultCode;
use codes::InnerResultInfo;
use codes::InnerResultRepeat;

// use web_controller::WebController;
// use web_controller_derive::WebController;

// #[path="../helpers.rs"]
// mod helpers; // why not work? How make it work? Or use like this is bad idea?

//curl -v -X GET -d '{"type": "hold", "v": "0", "amount": "10000", "client_id": "5", "paym_id": "6564565465464565646", "msisdn": "79267271941", "limit_type": "base"}' http://127.0.0.1:7878/test
//curl -v -X GET -d '{"type": "hold"}' http://127.0.0.1:7878/test
#[derive(Debug, Serialize, Deserialize)]
pub struct TestAsyncRequest {
    pub r#type: String,
    pub v: i64,
    // pub amount: i64,
    // pub client_id: Option<i64>,
    // pub paym_id: i64,
    // pub msisdn: i32,
    // pub limit_type: String, // base, ext, extprf
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TestAsyncResponse {
    pub code: InnerResultCode,
    pub info: InnerResultInfo,
    pub repeat: InnerResultRepeat,
    pub request_id: RequestId,
    pub payment_id: Option<i64>,
    pub tmp_str: String,
}

pub struct TestAsyncController {
    pub tmp_str: String,
    pub request_context: RequestContext,
    pub server_context: Arc<ServerContext>,
}

impl TestAsyncController {
    // Is it possible use new() from derive directly?
    pub async fn new(server_context: Arc<ServerContext>, request_context: RequestContext) -> Result<Self, std::io::Error> {
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

    pub async fn index(&self) -> Result<ControllerResponse<TestAsyncResponse>, std::io::Error> {
        // let req = &self.request;
        info!("TEST req_h {:?}", &self.request_context.request_parts);
        info!("TEST full_body {:?}", &self.request_context.full_body);
        let server_context = &self.server_context;
        let request_obj: TestAsyncRequest = match serde_json::from_slice(&self.request_context.full_body) {
            Ok(request_obj) => {
                request_obj
            },
            Err(e) => {
                warn!("Request parse error: {:?}", e);
                warn!("Request parse desc: {:?}", self.request_context.full_body);

                let response_result = InnerCode::OK;
                let response_obj = TestAsyncResponse {
                    code: response_result.code,
                    info: response_result.info,
                    repeat: response_result.repeat,
                    request_id: *&self.request_context.request_id,
                    payment_id: None,
                    tmp_str: "".to_string()
                };
                let controller_response = ControllerResponse {
                    response_obj: response_obj,
                    headers: HashMap::new(),
                };
                return Ok(controller_response);
                // return Err(std::io::Error::new(ErrorKind::Other, "oh no!"));
            }
        };
        info!("TEST test serde3 {:?}", request_obj);

        // let mut response = Response::new(Body::empty());
        //
        // *response.body_mut() = String::from("asds3432432d").into();
        // *response
        //     .header("Foo", "Bar")
        //     .status(StatusCode::NOT_FOUND);

        let response_result = InnerCode::OK;
        let response_obj = TestAsyncResponse {
            code: response_result.code,
            info: response_result.info,
            repeat: response_result.repeat,
            request_id: *&self.request_context.request_id,
            payment_id: None,
            tmp_str: "".to_string()
        };
        let controller_response = ControllerResponse {
            response_obj: response_obj,
            headers: HashMap::new(),
        };
        // let response = response_json!(&response_obj);
        // info!("TEST response {:?}", response);
        // info!("TEST controller_response {:?}", controller_response);

        Ok(controller_response)
    }

    // pub
}
