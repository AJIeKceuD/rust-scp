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

#[derive(Debug)]
pub struct LogsList {
    pub logs: Option<Vec<LogModel>>,
}

pub struct LogsHandler<'a> {
    pub server_context: Arc<ServerContext>,
    pub request_context: &'a RequestContext,
}

impl<'a> LogsHandler<'a> {
    pub async fn new(server_context: Arc<ServerContext>, request_context: &'a RequestContext) -> Result<LogsHandler<'a>, std::io::Error> {
        Ok(Self {
            request_context: request_context,
            server_context: server_context,
        })
    }

    pub async fn list(&self) -> Result<LogsList, std::io::Error> {
        let request_context = &self.request_context;
        let db_pool = &self.server_context.db_pool;
        let result;// = InnerResult::Ok( InnerResultElement {info: InnerResultInfo( String::from( InnerResultInfo::OK ) ), ..Default::default()} );
        let mut returned: LogsList = LogsList {
            logs: Option::None
        };

        // Log in-function
        let log = LogModel {
            id: Option::None,
            parent_id: Option::None,
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: Some(LogStage::Unknown.to_string()),
            log_type: LogType::Fn,
            name: LogName::Unknown,
            result: Option::None,
            http_code: Option::None,
            data: Some(format!("{:?}", (request_context))),
            basis: Option::None,
        };
        let log_id_fn = log_insert_db!(log, db_pool);
        // /Log in-function

        // loop {
            let result_temp = query_with_log!(
                db_pool,
                &request_context,

                "SELECT l_start.* FROM public.log l_start
LEFT JOIN public.log as l_end ON l_end.parent_id = l_start.id
WHERE l_start.parent_id IS NUll
ORDER BY l_start.id DESC
LIMIT $1;",

            10
            );

            println!("result_temp {:?}", result_temp);

            returned.logs = match result_temp {
                Ok(rows) => {
                    // debug!("record insert success: {:?}", row);
                    result = InnerResult::Ok(
                        InnerResultElement {
                            info: InnerResultInfo( String::from( InnerResultInfo::OK ) ),
                            detail: Some(String::from(&*format!("{:?}", rows)))
                        }
                    );

                    let mut logs_vec: Vec<LogModel> = Vec::new();
                    for row in rows.iter() {
                        logs_vec.push(LogModel {
                            id: Some(row.id),
                            parent_id: row.parent_id,
                            request_id: Some(RequestId(row.request_id)),
                            payment_id: row.payment_id,
                            stage: row.stage.clone(),
                            log_type: LogType::Unknown,
                            name: LogName::Unknown, //row.name.into()
                            result: row.result,
                            http_code: row.http_code,
                            data: row.data.clone(),
                            basis: row.basis.clone(),
                        });
                    };
                    Some(logs_vec)
                },
                Err(e) => {
                    result = InnerResult::ErrorUnknown(
                        InnerResultElement {
                            info: InnerResultInfo(String::from(InnerResultInfo::ERROR_UNKNOWN)),
                            detail: Some(String::from(&*format!("{:?}", e)))
                        }
                    );
                    Option::None
                }
            };

            // result = InnerResult::Ok( InnerResultElement {info: InnerResultInfo( String::from( InnerResultInfo::OK ) ), detail: Some(String::new())} );

        //     break;
        // }

        // Log in-function
        let log = LogModel {
            id: Option::None,
            parent_id: Some(log_id_fn),
            request_id: Some(request_context.request_id),
            payment_id: Option::None,
            stage: Some(LogStage::Unknown.to_string()),
            log_type: LogType::Fn,
            name: LogName::Unknown,
            result: Some(OuterResult::get_code(&result).0),
            http_code: Option::None,
            data: Some(format!("{:?}", returned)),
            basis: Option::None,
        };
        log_insert_db!(log, db_pool);
        // /Log in-function

        Ok(returned)
    }
}
