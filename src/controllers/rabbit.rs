use std::sync::Arc;
use std::str;
use hyper::{Request, Body, Response, HeaderMap};
use hyper::body::Bytes;
use hyper::http::request::Parts;
use log::{error, warn, info, debug, trace};

use crate::ServerContext;
// use serde_json::Result as serde_result;

// #[path="./../services/logs/db_log_object.rs"]
// mod db_log_object;
// use db_log_object::DBLogObject as DBLogObject;
use crate::router::model::log::{LogModel, RequestId};
use crate::router::RequestContext;
use amiquip::{Exchange, Publish};

// use web_controller::WebController;
// use web_controller_derive::WebController;

// #[path="../helpers.rs"]
// mod helpers; // why not work? How make it work? Or use like this is bad idea?

pub struct RabbitController {
    pub server_context: Arc<ServerContext>,
    pub request_context: RequestContext,
}

impl RabbitController {
    pub async fn new(server_context: Arc<ServerContext>, request_context: RequestContext) -> Result<Self, hyper::Error> {
        Ok(Self {
            server_context: server_context,
            request_context: request_context,
        })
    }

    pub async fn index(&self) -> Result<Response<Body>, hyper::Error> {
        // let req = &self.request;
        info!("TEST req_h {:?}", &self.request_context.request_parts);
        info!("TEST req_body {:?}", &self.request_context.full_body);
        let server_context = &self.server_context;

        let mut response = Response::new(Body::empty());

        *response.body_mut() = String::from("rabbit index").into();

        Ok(response)
    }

    pub async fn add(&self) -> Result<Response<Body>, hyper::Error> {
        // let req = &self.request;
        info!("TEST req_h {:?}", &self.request_context.request_parts);
        info!("TEST req_body {:?}", &self.request_context.full_body);
        let server_context = &self.server_context;

        // Open a channel - None says let the library choose the channel ID.
        // let channel = &server_context.rabbitmq_channel.open_channel(None).unwrap();

        // // Get a handle to the direct exchange on our channel.
        // let exchange = Exchange::direct(&server_context.rabbitmq_channel);
        //
        // // Publish a message to the "hello" queue.
        // exchange.publish(Publish::new("hello there".as_bytes(), "hello")).unwrap();
        //
        let mut response = Response::new(Body::empty());

        *response.body_mut() = String::from("rabbit add").into();

        Ok(response)
    }

    // pub
}
