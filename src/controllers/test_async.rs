use std::sync::Arc;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
// use serde_json::json;
use serde_json::{Value, json};

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

// use serde_json::Result as serde_result;

// #[path="./../services/logs/db_log_object.rs"]
// mod db_log_object;
// use db_log_object::DBLogObject as DBLogObject;
use crate::ServerContext;
use crate::model::log::{RequestId};
use crate::router::{RequestContext, ControllerResponse};

use crate::services::mapper::{
    inner_result::{
        InnerResult,
        InnerResultElement,
        InnerResultInfo,
        // InnerResultRepeat,
    },
    outer_result::{
        OuterResult,
        OuterResultCode,
        OuterResultInfo,
        OuterResultRepeat,
    },
};

use crate::services::record_register::RecordRegister;

// use web_controller::WebController;
// use web_controller_derive::WebController;

// #[path="../helpers.rs"]
// mod helpers; // why not work? How make it work? Or use like this is bad idea?

//curl -v -X GET -d '{"type": "hold", "v": "0", "amount": "10000", "client_id": "5", "paym_id": "6564565465464565646", "msisdn": "79267271941", "limit_type": "base"}' http://127.0.0.1:7878/test
//curl -v -X GET -d '{"type": "hold"}' http://127.0.0.1:7878/test
#[derive(Debug, Serialize, Deserialize)]
pub struct TestAsyncResponse {
    // pub result: InnerResult,
    pub code: OuterResultCode,
    pub info: OuterResultInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<OuterResultRepeat>,
    pub request_id: RequestId,
    pub payment_id: Option<i64>,
    pub tmp_str: String,
}

pub struct TestAsyncController {
    pub tmp_str: String,
    pub server_context: Arc<ServerContext>,
    pub request_context: RequestContext,
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

    pub(crate) async fn index(&self) -> Result<ControllerResponse, std::io::Error> {
        // let req = &self.request;
        info!("TEST req_h {:?}", &self.request_context.request_parts);
        info!("TEST full_body {:?}", &self.request_context.full_body);
        let server_context = &self.server_context;
        let response_result;// = InnerResult::Ok( InnerResultElement{info: InnerResultInfo(String::new()), ..Default::default()} );
        let request_body_value: Value;// = Value::Null;
        let response_obj ;

        loop {
            request_body_value = match serde_json::from_slice(&self.request_context.full_body) {
                Ok(request_obj) => {
                    request_obj
                },
                Err(e) => {
                    warn!("Request parse error: {:?}", e);
                    warn!("Request parse desc: {:?}", self.request_context.full_body);

                    response_result = InnerResult::ErrorIncomeData( InnerResultElement{info: InnerResultInfo ( String::from(InnerResultInfo::ERROR_INCOME_DATA_BAD_JSON) ), ..Default::default()} );

                    response_obj = TestAsyncResponse {
                        code: OuterResult::get_code(&response_result),
                        info: OuterResult::get_info(&response_result),
                        repeat: OuterResult::is_repeatable(&response_result),
                        request_id: *&self.request_context.request_id,
                        payment_id: None,
                        tmp_str: "".to_string()
                    };
                    break;
                    // Value::Null
                    // return Err(std::io::Error::new(ErrorKind::Other, "oh no!"));
                }
            };

            // let payment_register = PaymentRegister { request_context: *&self.request_context, server_context: *&self.server_context.clone() };
            let record_register = RecordRegister::new(server_context.clone(), &self.request_context ).await?;
            match request_body_value["v"].as_i64() { // TODO remove unwrap? or...
                Some(0) => {
                    response_result = record_register.process_v0().await?;

                    response_obj = TestAsyncResponse {
                        code: OuterResult::get_code(&response_result),
                        info: OuterResult::get_info(&response_result),
                        repeat: OuterResult::is_repeatable(&response_result),
                        request_id: *&self.request_context.request_id,
                        payment_id: None,
                        tmp_str: "".to_string()
                    };
                },
                _ => {
                    response_result = InnerResult::ErrorIncomeData(
                        InnerResultElement{
                            info: InnerResultInfo ( String::from(InnerResultInfo::ERROR_INCOME_DATA_BAD_VERSION)),
                            detail: Some(String::from("Value: ") + request_body_value["v"].to_string().as_str())
                        }
                    );

                    response_obj = TestAsyncResponse {
                        code: OuterResult::get_code(&response_result),
                        info: OuterResult::get_info(&response_result),
                        repeat: OuterResult::is_repeatable(&response_result),
                        request_id: *&self.request_context.request_id,
                        payment_id: None,
                        tmp_str: "".to_string()
                    };
                },
            };

            break;
        }

        // let request_value: Value = match serde_json::from_slice(&self.request_context.full_body) {
        //     Ok(request_obj) => {
        //         request_obj
        //     },
        //     Err(e) => {
        //         warn!("Request parse error: {:?}", e);
        //         warn!("Request parse desc: {:?}", self.request_context.full_body);
        //
        //         let response_result = InnerResult::ErrorIncomeDataParse( InnerResultElement{info: InnerResultInfo(String::new())} );
        //         let response_obj = TestAsyncResponse {
        //             result: response_result,
        //             // code: response_result.code,
        //             // info: response_result.info,
        //             // repeat: response_result.is_repeatable(),
        //             request_id: *&self.request_context.request_id,
        //             payment_id: None,
        //             tmp_str: "".to_string()
        //         };
        //         let controller_response = ControllerResponse {
        //             data: response_obj,
        //             headers: HashMap::new(),
        //         };
        //         return Ok(controller_response);
        //         // return Err(std::io::Error::new(ErrorKind::Other, "oh no!"));
        //     }
        // };

        // let request_obj: TestAsyncRequest = match serde_json::from_slice(&self.request_context.full_body) {
        //     Ok(request_obj) => {
        //         request_obj
        //     },
        //     Err(e) => {
        //         warn!("Request parse error: {:?}", e);
        //         warn!("Request parse desc: {:?}", self.request_context.full_body);
        //
        //         let response_result = InnerResult::ErrorIncomeData( InnerResultElement{info: InnerResultInfo(String::new())} );
        //         let response_obj = TestAsyncResponse {
        //             result: response_result,
        //             // code: response_result.code,
        //             // info: response_result.info,
        //             // repeat: response_result.is_repeatable(),
        //             request_id: *&self.request_context.request_id,
        //             payment_id: None,
        //             tmp_str: "".to_string()
        //         };
        //         let controller_response = ControllerResponse {
        //             data: response_obj,
        //             headers: HashMap::new(),
        //         };
        //         return Ok(controller_response);
        //         // return Err(std::io::Error::new(ErrorKind::Other, "oh no!"));
        //     }
        // };
        // info!("TEST test serde3 {:?}", request_obj);

        // let mut response = Response::new(Body::empty());
        //
        // *response.body_mut() = String::from("asds3432432d").into();
        // *response
        //     .header("Foo", "Bar")
        //     .status(StatusCode::NOT_FOUND);

        let response_obj = json!(response_obj);
        let controller_response = ControllerResponse {
            data: response_obj,
            headers: HashMap::new(),
            status: Option::None,
        };
        // let response = response_json!(&response_obj);
        // info!("TEST response {:?}", response);
        info!("TEST controller_response {:?}", controller_response);

        Ok(controller_response)
    }

    // pub
}
