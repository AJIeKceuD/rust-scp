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
    LogModelLine,
};

use crate::ServerContext;
use crate::router::{RequestContext};

use crate::services::mapper::outer_result::{OuterResult};

#[derive(Debug)]
pub struct LogsList {
    pub logs: Option<Vec<LogModelLine>>,
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
            stage: LogStage::Unknown,
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

                "SELECT 
                l_left.*, 

                l_right.id as right_id,
                l_right.parent_id as right_parent_id,
                l_right.request_id as right_request_id,
                l_right.payment_id as right_payment_id,
                l_right.stage as right_stage,
                l_right.type as right_log_type,
                l_right.name as right_name,
                l_right.result as right_result,
                l_right.http_code as right_http_code,
                l_right.data as right_data,
                l_right.basis as right_basis,
                l_right.create_at as right_create_at,
                l_right.update_at as right_update_at
                FROM public.log l_left
                LEFT JOIN public.log as l_right ON l_right.parent_id = l_left.id
                WHERE l_left.parent_id IS NULL OR l_left.parent_id NOT IN (SELECT id FROM public.log)
                ORDER BY l_left.id DESC
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

                    let mut logs_vec: Vec<LogModelLine> = Vec::new();
                    for row in rows.iter() {
                        // println!("temp {:?}", row.stage.clone());
                        // println!("temp {:?}", row.stage.clone());
                        // println!("temp {:?}", row.stage.clone().unwrap_or(LogType::Unknown.into()));
                        // println!("temp {:?}", row.stage.clone().unwrap_or(LogType::Unknown.into()).as_str());
                        // panic!("Ouch");

                        logs_vec.push(LogModelLine {
                            id: Some(row.id),
                            parent_id: row.parent_id,
                            request_id: Some(RequestId(row.request_id)),
                            payment_id: row.payment_id,
                            stage: row.stage.clone().unwrap_or(LogStage::Unknown.into()).as_str().into(),
                            log_type: row.r#type.clone().unwrap_or(LogType::Unknown.into()).as_str().into(),
                            name: row.name.clone().unwrap_or(LogName::Unknown.into()).as_str().into(),
                            result: row.result,
                            http_code: row.http_code,
                            data: row.data.clone(),
                            basis: row.basis.clone(),
                            create_at: row.create_at,
                            update_at: row.update_at,

                            right_id: row.right_id,
                            right_parent_id: row.right_parent_id,
                            right_request_id: Some(RequestId(row.right_request_id)),
                            right_payment_id: row.right_payment_id,
                            right_stage: row.right_stage.clone().unwrap_or(LogStage::Unknown.into()).as_str().into(),
                            right_log_type: row.right_log_type.clone().unwrap_or(LogStage::Unknown.into()).as_str().into(),
                            right_name: row.right_name.clone().unwrap_or(LogStage::Unknown.into()).as_str().into(),
                            right_result: row.right_result,
                            right_http_code: row.right_http_code,
                            right_data: row.right_data.clone(),
                            right_basis: row.right_basis.clone(),
                            right_create_at: row.right_create_at,
                            right_update_at: row.right_update_at,
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
            stage: LogStage::Unknown,
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
