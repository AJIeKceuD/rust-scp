use serde::{Serialize, Deserialize};
use strum_macros::Display;

pub struct LogId(i64);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct RequestId(pub Option<i64>);

struct PayId(i64);

#[derive(Display, Debug)]
pub enum LogStage {
    None,
    Init,
    Contract,
    Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogModel {
    pub request_id: RequestId,
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