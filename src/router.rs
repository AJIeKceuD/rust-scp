use std::{thread, time};
use std::sync::Arc;
use hyper::{Body, Request, Response, Method, StatusCode};
use futures::TryStreamExt as _; // map_ok()
use log::{error, warn, info, debug, trace};

use crate::ServerContext;
#[path="./controllers/test_async.rs"]
mod test_controller;
// use test_controller::TestController;
use test_controller::TestAsyncController;

#[path="./model/mod.rs"]
mod model;
use model::log::LogModel;
use model::log::LogStage;
use model::log::RequestId;

#[path="./middleware/mod.rs"]
mod middleware;
use middleware::request_id::RequestIdMiddleware;

#[derive(Debug)]
struct RequestContext {
    request_id: RequestId,
}

pub async fn router_handler(req: Request<Body>, server_context: Arc<ServerContext>) -> Result<Response<Body>, hyper::Error> {
    // Ok(Response::new("Hello, World".into()))
    let mut response = Response::new(Body::empty());
    println!("Thread ID: {:?}", thread::current().id());
    println!("{:?}", req);
    println!("{:?}", server_context.db_pool);

    // Split it here or deeper?
    let (request_parts, request_body) = req.into_parts();
    let full_body = hyper::body::to_bytes(request_body).await?;
    let mut request_context = RequestContext{
        request_id: RequestId(None),
    };

    // Middlewares...?

    // middleware for set global const like request_id, stage, ...
    request_context.request_id = RequestIdMiddleware::new(&server_context.db_pool).await;

    // log all request
    let log = LogModel {
        request_id: request_context.request_id,
        payment_id: Option::None,
        stage: LogStage::None.to_string(),
        log_type: LogStage::Init.to_string(),
        microtime_bgn: 0,
        microtime_end: 0,
        result: -1,
        http_code: -1,
        send_data: format!("{:?}", full_body),
        send_headers: format!("{:?}", request_parts),
        receive_data: "".into(),
        receive_headers: "".into(),
    };
    log_db!(log, &server_context.db_pool);
    // why not work async() in async func?

    // /Middlewares
    println!("{:?}", request_context);

    match (&request_parts.method, request_parts.uri.path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },

        (&Method::GET, "/test") => {
            // Await the full body to be concatenated into a single `Bytes`...
            // let full_body = hyper::body::to_bytes(req.into_body()).await?;

            // let mut controller = TestController{
            //     tmp_str: String::from("fd435"),
            //     request: req,
            //     server_context: server_context,
            // };
            let controller = TestAsyncController::new(server_context, request_parts, full_body).await?;
            response = controller.index().await?;
        },




/*
        (&Method::GET, "/echo") => {
            let ten_millis = time::Duration::from_millis(20000);
            thread::sleep(ten_millis);
            *response.body_mut() = req.into_body();
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
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
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    println!("{:?}", "Ok");

    Ok(response)
}