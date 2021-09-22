use serde::{Serialize, Deserialize};
use strum_macros::Display;

pub struct LogId(i64);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct RequestId(pub Option<i64>);

struct PayId(i64);

#[derive(Display, Debug)]
pub enum LogStage {
    Unknown,
    Init,
    Contract,
    Auth,
}

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum LogType {
    Unknown,
    Http,
    DB,
}

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum LogName {
    Unknown,
    RequestIn,
    DBSomeAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogModel {
    pub request_id: Option<RequestId>,
    pub payment_id: Option<i64>,
    pub stage: String,
    pub log_type: LogType,
    pub name: LogName,
    pub microtime_bgn: i64,
    pub microtime_end: i64,
    pub result: i32,
    pub http_code: i32,
    pub in_data: String,
    pub in_basis: String,
    pub out_data: String,
    pub out_basis: String,
}