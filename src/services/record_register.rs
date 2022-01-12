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
    LogModel,
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
        let log = LogModel {
            parent_id: Option::None,
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: LogStage::Unknown.to_string(),
            log_type: LogType::Fn,
            name: LogName::FnRecordRegister,
            result: Option::None,
            http_code: Option::None,
            data: format!("{:?}", (request_context)),
            basis: String::from(""),
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

            break;
        }

        // Log in-function
        let log = LogModel {
            parent_id: Some(log_id_fn),
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: LogStage::Unknown.to_string(),
            log_type: LogType::Fn,
            name: LogName::FnRecordRegister,
            result: Some(OuterResult::get_code(&result).0),
            http_code: Option::None,
            data: format!("{:?}", returned),
            basis: "".into(),
        };
        log_insert_db!(log, db_pool);
        // /Log in-function

        Ok(returned)
    }

    async fn hold(&self, hold_data: IncomeDataHold) -> OutcomeDataHold {
        let request_context = self.request_context;
        let db_pool = &self.server_context.db_pool;
        let mut result = InnerResult::Ok( InnerResultElement {info: InnerResultInfo( String::from( InnerResultInfo::OK ) ), ..Default::default()} );

        // Log in-function
        let log = LogModel {
            parent_id: Option::None,
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: LogStage::Unknown.to_string(),
            log_type: LogType::Fn,
            name: LogName::FnRecordHold,
            result: Option::None,
            http_code: Option::None,
            data: format!("{:?}", (hold_data)),
            basis: String::from(""),
        };
        let log_id_fn = log_insert_db!(log, db_pool);
        // /Log in-function

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

        // Log in-function
        let log = LogModel {
            parent_id: Some(log_id_fn),
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: LogStage::Unknown.to_string(),
            log_type: LogType::Fn,
            name: LogName::FnRecordHold,
            result: Some(OuterResult::get_code(&result).0),
            http_code: Option::None,
            data: format!("{:?}", hold_result),
            basis: "".into(),
        };
        log_insert_db!(log, db_pool);
        // /Log in-function

        hold_result
    }
}
