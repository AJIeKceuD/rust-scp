use serde::{Serialize, Deserialize};
use strum_macros::Display;
use chrono::{DateTime, TimeZone, Utc};
use chrono::serde::ts_seconds_option;

pub struct LogId(i64);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct RequestId(pub Option<i64>);

#[derive(Serialize, Deserialize, Display, Debug)]
#[allow(dead_code)]
pub enum LogStage {
    Unknown,
    Init,
    Contract,
    Auth,
}

impl From<&str> for LogStage {
    fn from(item: &str) -> Self {
        match item {
            "Unknown" => LogStage::Unknown,
            "Init" => LogStage::Init,
            "Contract" => LogStage::Contract,
            "Auth" => LogStage::Auth,
            _ => LogStage::Unknown
        }
    }
}

impl From<LogStage> for String {
    fn from(item: LogStage) -> Self {
        match item {
            LogStage::Unknown => String::from("Unknown"),
            LogStage::Init => String::from("Init"),
            LogStage::Contract => String::from("Contract"),
            LogStage::Auth => String::from("Auth"),
        }
    }
}

#[derive(Serialize, Deserialize, Display, Debug)]
#[allow(dead_code)]
pub enum LogType {
    Unknown,
    Http,
    DB,
    Fn,
    Milestone,
}

impl From<&str> for LogType {
    fn from(item: &str) -> Self {
        match item {
            "Unknown" => LogType::Unknown,
            "Http" => LogType::Http,
            "DB" => LogType::DB,
            "Fn" => LogType::Fn,
            "Milestone" => LogType::Milestone,
            _ => LogType::Unknown
        }
    }
}

impl From<LogType> for String {
    fn from(item: LogType) -> Self {
        match item {
            LogType::Unknown => String::from("Unknown"),
            LogType::Http => String::from("Http"),
            LogType::DB => String::from("DB"),
            LogType::Fn => String::from("Fn"),
            LogType::Milestone => String::from("Milestone")
        }
    }
}

#[derive(Serialize, Deserialize, Display, Debug)]
#[allow(dead_code)]
pub enum LogName {
    Unknown,
    RequestIn,
    // DB, // use Unknown?
    DBRecordHold,
    // Fn,
    FnRecordRegister,
    FnRecordHold,
}

impl From<&str> for LogName {
    fn from(item: &str) -> Self {
        match item {
            "Unknown" => LogName::Unknown,
            "RequestIn" => LogName::RequestIn,
            "DBRecordHold" => LogName::DBRecordHold,
            "FnRecordRegister" => LogName::FnRecordRegister,
            "FnRecordHold" => LogName::FnRecordHold,
            _ => LogName::Unknown
        }
    }
}

impl From<LogName> for String {
    fn from(item: LogName) -> Self {
        match item {
            LogName::Unknown => String::from("Unknown"),
            LogName::RequestIn => String::from("RequestIn"),
            LogName::DBRecordHold => String::from("DBRecordHold"),
            LogName::FnRecordRegister => String::from("FnRecordRegister"),
            LogName::FnRecordHold => String::from("FnRecordHold")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogModel {
    pub id: Option<i64>,
    pub parent_id: Option<i64>,
    pub request_id: Option<RequestId>,
    pub payment_id: Option<i64>,
    pub stage: LogStage,
    pub log_type: LogType,
    pub name: LogName,
    // pub microtime: i64,
    pub result: Option<i32>,
    pub http_code: Option<i32>,
    pub data: Option<String>,
    pub basis: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogModelLine {
    pub id: Option<i64>,
    pub parent_id: Option<i64>,
    pub request_id: Option<RequestId>,
    pub payment_id: Option<i64>,
    pub stage: LogStage,
    pub log_type: LogType,
    pub name: LogName,
    // pub microtime: i64,
    pub result: Option<i32>,
    pub http_code: Option<i32>,
    pub data: Option<String>,
    pub basis: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub create_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub update_at: Option<DateTime<Utc>>,

    pub right_id: Option<i64>,
    pub right_parent_id: Option<i64>,
    pub right_request_id: Option<RequestId>,
    pub right_payment_id: Option<i64>,
    pub right_stage: LogStage,
    pub right_log_type: LogType,
    pub right_name: LogName,
    // pub microtime: i64,
    pub right_result: Option<i32>,
    pub right_http_code: Option<i32>,
    pub right_data: Option<String>,
    pub right_basis: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub right_create_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub right_update_at: Option<DateTime<Utc>>,
}
