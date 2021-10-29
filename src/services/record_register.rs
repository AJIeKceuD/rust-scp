use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use sqlx::{Row};

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

use crate::services::mapper::{
    inner_result::{
        InnerResult,
        InnerResultElement,
        InnerResultInfo,
        // InnerResultRepeat,
    },
};
use crate::model::log::{
    // LogModel,
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
pub struct OutcomeDataProcess {
    pub id: Option<i64>,
}

#[derive(Debug)]
pub struct IncomeDataHold {
    pub amount: i32,
    // pub client_id: Option<i64>,
    // pub paym_id: i64,
    // pub msisdn: i32,
    // pub limit_type: String, // base, ext, extprf
}
#[derive(Debug)]
pub struct OutcomeDataHold {
    pub id: Option<i64>,
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

    pub async fn process_v0(&self) -> Result<OutcomeDataProcess, std::io::Error> {
        let request_context = &self.request_context;
        let db_pool = &self.server_context.db_pool;
        let result;// = InnerResult::Ok( InnerResultElement {info: InnerResultInfo( String::from( InnerResultInfo::OK ) ), ..Default::default()} );
        let mut returned: OutcomeDataProcess = OutcomeDataProcess {
            id: Option::None
        };

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

                    let hold_result = self.hold(hold_data).await;
                    returned.id = hold_result.id;

                    result = InnerResult::Ok( InnerResultElement {info: InnerResultInfo( String::from( InnerResultInfo::OK ) ), detail: Some(format!("{:?}", hold_result))} );
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

        Ok(returned)
    }

    async fn hold(&self, hold_data: IncomeDataHold) -> OutcomeDataHold {
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

        // let record_id;
        // sqlx::query!(
        //     "INSERT INTO record (
        //         outer_id,
        //         stage,
        //         sum,
        //         tmp
        //     )
        //     VALUES ($1, $2, $3)
        //     RETURNING id;",
        //
        //     Option::None::<i64>,
        //     String::from("hold"),
        //     hold_data.amount,
        //
        //     record_id
        // )
        // .fetch_one(db_pool);

        let result_temp = query_with_log!(
            db_pool,
            &request_context,

            "INSERT INTO record (
                outer_id,
                stage,
                sum
            )
            VALUES ($1, $2, $3)
            RETURNING id;",

            Option::None::<i64>,
            String::from("hold"),
            hold_data.amount
        );

        let hold_result: OutcomeDataHold = match result_temp {
            Ok(row) => {
                // debug!("record insert success: {:?}", row);
                result = InnerResult::Ok(
                    InnerResultElement {
                        info: InnerResultInfo( String::from( InnerResultInfo::OK ) ),
                        detail: Some(String::from(&*format!("{:?}", row)))
                    }
                );
                OutcomeDataHold {
                    id: Some(row.id),
                }
            },
            Err(e) => {
                result = InnerResult::ErrorUnknown(
                    InnerResultElement {
                        info: InnerResultInfo(String::from(InnerResultInfo::ERROR_UNKNOWN)),
                        detail: Some(String::from(&*format!("{:?}", e)))
                    }
                );
                OutcomeDataHold {
                    id: Option::None,
                }
            }
        };

        // let db_query = "
        // INSERT INTO record (
        //     outer_id,
        //     stage,
        //     sum,
        //     tmp
        // )
        // VALUES ($1, $2, $3)
        // RETURNING id;
        // ";
        // let db_data: (Option<i64>, String, i32) = (
        //     Option::None::<i64>,
        //     String::from("hold"),
        //     hold_data.amount
        // );
        //
        // let log = LogModelIn {
        //     request_id: Some(request_context.request_id),
        //     payment_id: Option::None,
        //     stage: LogStage::Unknown.to_string(),
        //     log_type: LogType::DB,
        //     name: LogName::DBRecordHold,
        //     in_data: format!("{:?}", db_data),
        //     in_basis: String::from(db_query),
        // };
        // let log_id = log_insert_db!(log, db_pool);
        //
        // let (db_data_a, db_data_b, db_data_c) = db_data;
        // let db_request =
        //     sqlx::query(&db_query)
        //         .bind(db_data_a)
        //         .bind(db_data_b)
        //         .bind(db_data_c)
        //         .fetch_one(db_pool)
        //         .await;
        // debug!("record insert await");
        //
        // let mut record_id: i64 = 0;
        // match &db_request {
        //     Ok(row) => {
        //         // debug!("record insert success: {:?}", row);
        //         record_id = row.get("id");
        //     },
        //     Err(e) => {
        //         error!("db error while record insert: {:?}", e);
        //         result = InnerResult::ErrorUnknown(
        //             InnerResultElement {
        //                 info: InnerResultInfo(String::from(InnerResultInfo::ERROR_UNKNOWN)),
        //                 detail: Some(String::from(&*format!("{:?}", e)))
        //             }
        //         );
        //     }
        // };
        //
        // debug!("record insert result: {:?}", record_id);
        //
        // let log = LogModelOut {
        //     payment_id: None,
        //     result: Option::None,
        //     http_code: Option::None,
        //     out_data: format!("ID: {:?}, result {:?}", record_id, result),
        //     out_basis: "".into(),
        // };
        // log_update_db!(log, db_pool, log_id);

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

        hold_result
    }
}
