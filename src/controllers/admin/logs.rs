use std::sync::Arc;
use std::collections::HashMap;
// use std::collections::BTreeMap;
// use handlebars::{
//     to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
// };

use tera::{Context, Result as TeraResult, Tera};
use crate::TEMPLATES;

use serde::{Deserialize, Serialize};
// use serde_json::json;
use serde_json::{Map, Value, json};

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

use crate::ServerContext;
use crate::model::log::{RequestId};
use crate::router::{RequestContext, ControllerResponse};

use crate::services::mapper::{
    inner_result::{
        InnerResult,
        InnerResultElement,
        InnerResultInfo,
        // InnerResultRepeat,
    },
    outer_result::{
        OuterResult,
        OuterResultCode,
        OuterResultInfo,
        OuterResultRepeat,
    },
};

use crate::services::logs::handler::LogsHandler;
use crate::services::logs::handler::LogsList;

//curl -v -X GET -d '{"type": "hold", "v": "0", "amount": "10000", "client_id": "5", "paym_id": "6564565465464565646", "msisdn": "79267271941", "limit_type": "base"}' http://127.0.0.1:7878/test
//curl -v -X GET -d '{"type": "hold"}' http://127.0.0.1:7878/test
#[derive(Debug, Serialize, Deserialize)]
pub struct LogsResponse {
    // pub result: InnerResult,
    pub code: OuterResultCode,
    pub info: OuterResultInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<OuterResultRepeat>,
    pub request_id: RequestId,
    pub record_id: Option<i64>,
    pub tmp_str: String,
}

pub struct LogsController<'a> {
    pub tmp_str: String,
    pub server_context: Arc<ServerContext>,
    pub request_context: &'a RequestContext,
}

impl <'a> LogsController<'a> {
    pub async fn new(server_context: Arc<ServerContext>, request_context: &'a RequestContext) -> Result<LogsController<'a>, std::io::Error> {
        Ok(Self {
            tmp_str: String::from("fd435"),
            request_context: request_context,
            server_context: server_context,
        })
    }

    pub(crate) async fn index(&self) -> Result<ControllerResponse, std::io::Error> {
        // let req = &self.request;
        debug!("TEST req_h {:?}", &self.request_context.request_parts);
        debug!("TEST full_body {:?}", &self.request_context.full_body);
        let server_context = &self.server_context;
        // let response_result;// = InnerResult::Ok( InnerResultElement{info: InnerResultInfo(String::new()), ..Default::default()} );
        let request_body_value: Value;// = Value::Null;
        let response_obj;
        let mut logs_list: LogsList = LogsList{
            logs: Option::None
        };

        loop {
            // request_body_value = match serde_json::from_slice(&self.request_context.full_body) {
            //     Ok(request_obj) => {
            //         request_obj
            //     },
            //     Err(e) => {
            //         warn!("Request parse error: {:?}", e);
            //         warn!("Request parse desc: {:?}", self.request_context.full_body);

            //         response_result = InnerResult::ErrorIncomeData(
            //             InnerResultElement {
            //                 info: InnerResultInfo( String::from( InnerResultInfo::ERROR_INCOME_DATA_BAD_JSON) ),
            //                 ..Default::default()
            //             }
            //         );

            //         response_obj = LogsResponse {
            //             code: OuterResult::get_code(&response_result),
            //             info: OuterResult::get_info(&response_result),
            //             repeat: OuterResult::is_repeatable(&response_result),
            //             request_id: *&self.request_context.request_id,
            //             record_id: None,
            //             tmp_str: "".to_string()
            //         };
            //         break;
            //     }
            // };

            let logs_handler = LogsHandler::new(server_context.clone(), &self.request_context ).await?;
            // match request_body_value["v"].as_i64() {
            //     Some(0) => {
                    logs_list = logs_handler.list().await?;
                    // info!("{:?}", logs_list);

                    // response_result = InnerResult::Ok(
                    //     InnerResultElement{
                    //         info: InnerResultInfo(String::from( InnerResultInfo::OK )),
                    //         detail: Some(format!("{:?}", logs_list)),
                    //     }
                    // );

                    // response_obj = LogsResponse {
                    //     code: OuterResult::get_code(&response_result),
                    //     info: OuterResult::get_info(&response_result),
                    //     repeat: OuterResult::is_repeatable(&response_result),
                    //     request_id: *&self.request_context.request_id,
                    //     record_id: Some(0),
                    //     tmp_str: "".to_string()
                    // };
            //     },
            //     _ => {
            //         response_result = InnerResult::ErrorIncomeData(
            //             InnerResultElement{
            //                 info: InnerResultInfo ( String::from(InnerResultInfo::ERROR_INCOME_DATA_BAD_VERSION)),
            //                 detail: Some(String::from("Value: ") + request_body_value["v"].to_string().as_str())
            //             }
            //         );

            //         response_obj = LogsResponse {
            //             code: OuterResult::get_code(&response_result),
            //             info: OuterResult::get_info(&response_result),
            //             repeat: OuterResult::is_repeatable(&response_result),
            //             request_id: *&self.request_context.request_id,
            //             record_id: None,
            //             tmp_str: "".to_string()
            //         };
            //     },
            // };

            break;
        }

        /*
        // let response_obj = json!(response_obj);
        // create the handlebars registry
        let mut handlebars = Handlebars::new();

        // register template from a file and assign a name to it
        // TODO normal match

        match handlebars.register_template_file("table", "./src/views/admin/logs.hbs") {
            // () => {
            //     ()
            // },
            _ => {

            }
        }

        // register some custom helpers
        handlebars.register_helper("format", Box::new(format_helper));
        // handlebars.register_helper("ranking_label", Box::new(rank_helper));

        // make data and render it
        let mut handlebars_data = Map::new();
        handlebars_data.insert("records".to_string(), to_json(&logs_list.logs));
        let response_obj = match handlebars.render("table", &handlebars_data) {
            Ok(value) =>  {
                value
            },
            Err(e) => {
                e.to_string()
            }
        };
        println!("{}", "response_obj");
        println!("{}", response_obj);
        // /create the handlebars
        */

        let mut context = Context::new();
        context.insert("username", &"Bob");
        context.insert("numbers", &vec![1, 2, 3]);
        context.insert("show_all", &false);
        context.insert("bio", &"<script>alert('pwnd');</script>");
        context.insert("records", &logs_list.logs);

        // A one off template
        Tera::one_off("hello", &Context::new(), true).unwrap();

        response_obj = match TEMPLATES.render("admin/logs.html", &context) {
            Ok(s) => {
                // println!("{:?}", s);
                s
            },
            Err(e) => {
                println!("Tera Error: {}", e);
                e.to_string()
                // let mut cause = e.source();
                // while let Some(e) = cause {
                //     println!("Reason: {}", e);
                //     cause = e.source();
                // }
            }
        };

        let controller_response = ControllerResponse {
            data: json!(response_obj),
            headers: HashMap::new(),
            status: Option::None,
        };
        // let response = response_json!(&response_obj);
        // debug!("TEST response {:?}", response);
        // debug!("TEST controller_response {:?}", controller_response);

        Ok(controller_response)
    }
}
