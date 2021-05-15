use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DBLogObject {
    pub request_id: i64,
    pub payment_id: i64,
    pub stage: String,
    pub log_type: String,
    pub microtime_bgn: i64,
    pub microtime_end: i64,
    pub result: i32,
    pub http_code: i32,
    pub send_data: String,
    pub send_headers: String,
    pub receive_data: String,
    pub receive_headers: String,
}