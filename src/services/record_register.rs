use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Utc, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use sqlx::postgres::PgArguments;
use sqlx::postgres::PgRow;
use sqlx::{Row};

use log::{error, warn, info, debug, trace};

use crate::services::mapper::{
    inner_result::{
        InnerResult,
        InnerResultElement,
        InnerResultInfo,
        InnerResultRepeat,
    },
};
use crate::model::log::{
    LogModel,
    LogStage,
    LogType,
    LogName,
    RequestId,
    LogModelOut,
    LogModelIn,
};

use crate::ServerContext;
use crate::router::{RequestContext};

use crate::services::mapper::outer_result::{OuterResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeDataV0 {
    pub r#type: String,
    pub v: i64,
    pub amount: i32,
    // pub client_id: Option<i64>,
    // pub paym_id: i64,
    // pub msisdn: i32,
    // pub limit_type: String, // base, ext, extprf
}

#[derive(Debug)]
pub struct IncomeDataHold {
    pub amount: i32,
    // pub client_id: Option<i64>,
    // pub paym_id: i64,
    // pub msisdn: i32,
    // pub limit_type: String, // base, ext, extprf
}

pub struct RecordRegister<'a> {
    pub server_context: Arc<ServerContext>,
    pub request_context: &'a RequestContext,
}

impl<'a> RecordRegister<'a> {
    pub async fn new(server_context: Arc<ServerContext>, request_context: &'a RequestContext) -> Result<RecordRegister<'a>, std::io::Error> {
        Ok(Self {
            request_context: request_context,
            server_context: server_context,
        })
    }

    pub async fn process_v0(&self) -> Result<InnerResult, std::io::Error> {
        let request_context = &self.request_context;
        let db_pool = &self.server_context.db_pool;
        let mut result = InnerResult::Ok( InnerResultElement {info: InnerResultInfo( String::from( InnerResultInfo::OK ) ), ..Default::default()} );

        // Log in-function
        let log = LogModelIn {
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: LogStage::Unknown.to_string(),
            log_type: LogType::Fn,
            name: LogName::FnRecordRegister,
            in_data: format!("{:?}", (request_context)),
            in_basis: String::from(""),
        };
        let log_id_fn = log_insert_db!(log, db_pool);
        // /Log in-function

        loop {
            let request_obj: IncomeDataV0 = match serde_json::from_slice(&request_context.full_body) {
                Ok(request_obj) => {
                    request_obj
                },
                Err(e) => {
                    warn!("Request parse error: {:?}", &e);
                    warn!("Request parse desc: {:?}", self.request_context.full_body);

                    result = InnerResult::ErrorIncomeData(InnerResultElement { info: InnerResultInfo(InnerResultInfo::ERROR_INCOME_DATA.to_string()), detail: Some(format!("{:?}", &e)) });
                    break;
                    // return Err(std::io::Error::new(ErrorKind::Other, "oh no!"));
                }
            };

            match &request_obj.r#type[..] {
                "hold" => {
                    let hold_data = IncomeDataHold {
                        amount: request_obj.amount,
                    };
                    result = self.hold(hold_data).await;
                }
                _ => {
                    result = InnerResult::ErrorIncomeData(InnerResultElement { info: InnerResultInfo(InnerResultInfo::ERROR_INCOME_DATA.to_string()), ..Default::default() });
                    break;
                }
            }

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

            break;
        }

        // Log in-function
        let log = LogModelOut {
            payment_id: Option::None,
            result: Some(OuterResult::get_code(&result).0),
            http_code: Option::None,
            out_data: format!("{:?}", result),
            out_basis: "".into(),
        };
        log_update_db!(log, db_pool, log_id_fn);
        // /Log in-function

        Ok(result)
    }

    async fn hold(&self, hold_data: IncomeDataHold) -> InnerResult {
        let request_context = &self.request_context;
        let db_pool = &self.server_context.db_pool;
        let mut result = InnerResult::Ok( InnerResultElement {info: InnerResultInfo( String::from( InnerResultInfo::OK ) ), ..Default::default()} );

        // Log in-function
        let log = LogModelIn {
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: LogStage::Unknown.to_string(),
            log_type: LogType::Fn,
            name: LogName::FnRecordHold,
            in_data: format!("{:?}", (hold_data)),
            in_basis: String::from(""),
        };
        let log_id_fn = log_insert_db!(log, db_pool);
        // /Log in-function

        let db_query = "
        INSERT INTO record (
            outer_id,
            stage,
            sum
        )
        VALUES ($1, $2, $3)
        RETURNING id;
        ";
        let db_data: (Option<i64>, String, i32) = (
            Option::None::<i64>,
            String::from("hold"),
            hold_data.amount
        );

        let log = LogModelIn {
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: LogStage::Unknown.to_string(),
            log_type: LogType::DB,
            name: LogName::DBRecordHold,
            in_data: format!("{:?}", db_data),
            in_basis: String::from(db_query),
        };
        let log_id = log_insert_db!(log, db_pool);

        let (db_data_a, db_data_b, db_data_c) = db_data;
        let db_request =
            sqlx::query(&db_query)
                .bind(db_data_a)
                .bind(db_data_b)
                .bind(db_data_c)
                .fetch_one(db_pool)
                .await;
        debug!("record insert await");

        let mut record_id: i64 = 0;
        match &db_request {
            Ok(row) => {
                // debug!("record insert success: {:?}", row);
                record_id = row.get("id");
            },
            Err(e) => {
                error!("db error while record insert: {:?}", e);
                result = InnerResult::ErrorUnknown(
                    InnerResultElement {
                        info: InnerResultInfo(String::from(InnerResultInfo::ERROR_UNKNOWN)),
                        detail: Some(String::from(&*format!("{:?}", e)))
                    }
                );
            }
        };

        debug!("record insert result: {:?}", record_id);

        let log = LogModelOut {
            payment_id: None,
            result: Option::None,
            http_code: Option::None,
            out_data: format!("ID: {:?}, result {:?}", record_id, result),
            out_basis: "".into(),
        };
        log_update_db!(log, db_pool, log_id);

        // Log in-function
        let log = LogModelOut {
            payment_id: Option::None,
            result: Some(OuterResult::get_code(&result).0),
            http_code: Option::None,
            out_data: format!("{:?}", result),
            out_basis: "".into(),
        };
        log_update_db!(log, db_pool, log_id_fn);
        // /Log in-function

        result
    }
}
