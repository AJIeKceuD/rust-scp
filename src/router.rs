use std::{thread, time};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use hyper::{Body, Request, Response, Method, StatusCode};
use hyper::body::Bytes;
use hyper::http::request::Parts;
use futures::TryStreamExt as _; // map_ok()
use log::{error, warn, info, debug, trace};

use crate::ServerContext;
#[path="./controllers/test_async.rs"]
mod test_controller;
// use test_controller::TestController;
use test_controller::TestAsyncController;

#[path="./controllers/rabbit.rs"]
mod rabbit_controller;
// use test_controller::TestController;
use rabbit_controller::RabbitController;

#[path="./model/mod.rs"]
mod model;
use model::log::LogModel;
use model::log::LogStage;
use model::log::RequestId;

#[path="./middleware/mod.rs"]
mod middleware;
use middleware::request_id::RequestIdMiddleware;
use crate::router::model::log::{LogType, LogName};
use chrono::{Utc, Local};

#[derive(Debug)]
pub struct RequestContext {
    request_id: RequestId,
    request_parts: Parts,
    full_body: Bytes,
    stage: Option<LogStage>,
}

#[derive(Debug)]
struct ControllerResponse<T> {
    response_obj: T,
    headers: HashMap<String, String>,
}

pub async fn router_handler(req: Request<Body>, server_context: Arc<ServerContext>) -> Result<Response<Body>, std::io::Error> {
    // Ok(Response::new("Hello, World".into()))
    let mut response = Response::new(Body::empty());
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let since_the_epoch_in_ms = since_the_epoch.as_secs() as i64 * 1000 +
        since_the_epoch.subsec_nanos() as i64 / 1_000_000;
    println!("since_the_epoch {:?}", since_the_epoch);
    println!("since_the_epoch_in_ms {:?}", since_the_epoch_in_ms);
    println!("Thread ID: {:?}", thread::current().id());
    println!("{:?}", req);
    println!("{:?}", server_context.db_pool);

    // Split it here or deeper?
    let (request_parts, request_body) = req.into_parts();
    let full_body = hyper::body::to_bytes(request_body).await.unwrap(); // TODO remove unwrap
    let mut request_context = RequestContext{
        request_id: RequestId(None),
        request_parts: request_parts,
        full_body: full_body,
        stage: None
    };

    // Middlewares...?

    // middleware for set global const like request_id, stage, ...
    request_context.request_id = RequestIdMiddleware::new(&server_context.db_pool).await;

    // log all request
    let log = LogModel {
        request_id: Some(request_context.request_id),
        payment_id: Option::None,
        stage: LogStage::Unknown.to_string(),
        log_type: LogType::Http,
        name: LogName::RequestIn,
        microtime_bgn: since_the_epoch_in_ms,
        microtime_end: 0,
        result: -1,
        http_code: -1,
        in_data: format!("{:?}", request_context.full_body),
        in_basis: format!("{:?}", request_context.request_parts),
        out_data: "".into(),
        out_basis: "".into(),
    };
    let log_id = log_insert_db!(log, &server_context.db_pool);
    // why not work async() in async func?

    // /Middlewares
    println!("{:?}", request_context);

    match (&request_context.request_parts.method, request_context.request_parts.uri.path()) {
        (&Method::GET, "/") => {
            // *response.body_mut() = Body::from("Try POSTing data to /echo");
        },

        (&Method::GET, "/test") => {
            // Await the full body to be concatenated into a single `Bytes`...
            // let full_body = hyper::body::to_bytes(req.into_body()).await?;

            // let mut controller = TestController{
            //     tmp_str: String::from("fd435"),
            //     request: req,
            //     server_context: server_context,
            // };
            let controller = TestAsyncController::new(server_context.clone(), request_context).await?;
            let controller_response = controller.index().await?;

            response = response_json!(&controller_response.response_obj);
            info!("TEST response {:?}", response);
        },

        (&Method::GET, "/rabbit") => {
            // let controller = RabbitController::new(server_context.clone(), request_context).await?;
            // response = controller.index().await?;
        },

        (&Method::GET, "/rabbit/add") => {
            // let controller = RabbitController::new(server_context.clone(), request_context).await?;
            // response = controller.add().await?;
        },




        (&Method::GET, "/echo") => {
            let ten_millis = time::Duration::from_millis(20000);
            thread::sleep(ten_millis);
            let controller = TestAsyncController::new(server_context.clone(), request_context).await?;
            let controller_response = controller.index().await?;
        },
        (&Method::POST, "/echo") => {
            let controller = TestAsyncController::new(server_context.clone(), request_context).await?;
            let controller_response = controller.index().await?;
        },

        /*
                // Yet another route inside our match block...
                (&Method::POST, "/echo/uppercase") => {
                    // This is actually a new `futures::Stream`...
                    let mapping = req
                        .into_body()
                        .map_ok(|chunk| {
                            chunk.iter()
                                .map(|byte| byte.to_ascii_uppercase())
                                .collect::<Vec<u8>>()
                        });

                    // Use `Body::wrap_stream` to convert it to a `Body`...
                    *response.body_mut() = Body::wrap_stream(mapping);
                },
                // Yet another route inside our match block...
                (&Method::POST, "/echo/reverse") => {
                    // Await the full body to be concatenated into a single `Bytes`...
                    let full_body = hyper::body::to_bytes(req.into_body()).await?;

                    // Iterate the full body in reverse order and collect into a new Vec.
                    let reversed = full_body.iter()
                        .rev()
                        .cloned()
                        .collect::<Vec<u8>>();

                    *response.body_mut() = reversed.into();
                },

         */
        _ => {
            // *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let since_the_epoch_in_ms = since_the_epoch.as_secs() as i64 * 1000 +
        since_the_epoch.subsec_nanos() as i64 / 1_000_000;

    let (response_parts, response_body) = response.into_parts();
    let log = LogModel {
        request_id: None, // no need. We diff RequestID::None and None. RequestID::None mean we must set Null, None mean change no need.
        payment_id: None,
        stage: LogStage::Unknown.to_string(), // no need
        log_type: LogType::Http, // no need
        name: LogName::RequestIn, // no need
        microtime_bgn: 0, // no need
        microtime_end: since_the_epoch_in_ms,
        result: 0, // ???
        http_code: response_parts.status.as_u16().into(),
        in_data: "".into(), // no need
        in_basis: "".into(), // no need
        out_data: format!("{:?}", response_body),
        out_basis: format!("{:?}", response_parts),
    };
    log_update_db!(log, &server_context.db_pool, log_id);
    let response = Response::from_parts(response_parts, response_body);

    println!("{:?}", "Ok");

    Ok(response)
}