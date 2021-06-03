use std::{thread, time};
use std::sync::Arc;
use hyper::{Body, Request, Response, Method, StatusCode};
use futures::TryStreamExt as _; // map_ok()

#[path="./controllers/test_async.rs"]
mod test_controller;
// use test_controller::TestController;
use test_controller::TestAsyncController as TestController;
use crate::ServerContext;

pub async fn router_handler(req: Request<Body>, server_context: Arc<ServerContext>) -> Result<Response<Body>, hyper::Error> {
    // Ok(Response::new("Hello, World".into()))
    let mut response = Response::new(Body::empty());
    println!("Thread ID: {:?}", thread::current().id());
    println!("{:?}", req);
    println!("{:?}", server_context.db_pool);

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },

        (&Method::GET, "/test") => {
            // Await the full body to be concatenated into a single `Bytes`...
            // let full_body = hyper::body::to_bytes(req.into_body()).await?;

            let mut controller = TestController{
                tmp_str: String::from("fd435"),
                request: req,
                server_context: server_context,
            };
            controller.new();
            response = controller.default().await?;
        },





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
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    println!("{:?}", "Ok");

    Ok(response)
}