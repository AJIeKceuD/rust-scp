use crate::router::model::log::RequestId;
use sqlx::{Pool, Postgres};
use log::{error, warn, info, debug, trace};

pub struct RequestIdMiddleware {
    pub request_id: i64
}

impl RequestIdMiddleware {
    pub async fn new(pool: &Pool<Postgres>) -> RequestId {
        let db_request = sqlx::query!("SELECT nextval('api_request_id_seq') as id;")
            .fetch_one(pool);
        debug!("request id await");

        let request_id = match db_request.await {
            Ok(row) => {
                debug!("db success while request_id: {:?}", row);
                RequestId(row.id)
            },
            Err(e) => {
                error!("db error while request_id: {:?}", e);
                RequestId(None)
            }
        };

        debug!("request id result: {:?}", request_id);

        request_id
    }
}