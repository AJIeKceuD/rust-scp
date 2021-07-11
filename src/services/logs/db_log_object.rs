use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Display, Debug)]
#[allow(dead_code)]
pub enum DBLogStage_t {
    None,
    Init,
    Contract,
    Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBLogObject_t {
    pub request_id: i64,
    pub payment_id: Option<i64>,
    pub stage: String,
    pub log_type: String,
    pub microtime_bgn: i64,
    pub microtime_end: i64,
    pub result: i32,
    pub http_code: i32,
    pub send_data: String, // out_data
    pub send_headers: String, // out_basis
    pub receive_data: String, // in_data
    pub receive_headers: String, // in_basis
}