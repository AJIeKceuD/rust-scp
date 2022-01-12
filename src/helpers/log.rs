#[macro_export]
macro_rules! log_insert_db {
    ($log_object:expr, $pool:expr) => {
        {
        // block_on(
        //     async {
                let since_the_epoch = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                let since_the_epoch_in_ms = since_the_epoch.as_secs() as i64 * 1000 +
                    since_the_epoch.subsec_nanos() as i64 / 1_000_000;
                println!("since_the_epoch {:?}", since_the_epoch);
                println!("since_the_epoch_in_ms {:?}", since_the_epoch_in_ms);

                let log_object = $log_object;

                // why not work?
                // debug!("log insert data: {:?}", (
                //     log_object.request_id,
                //     log_object.payment_id,
                //     log_object.stage,
                //     log_object.type,
                //     log_object.microtime_bgn,
                //     log_object.microtime_end,
                //     log_object.result,
                //     log_object.http_code,
                //     log_object.send_data,
                //     log_object.send_headers,
                //     log_object.receive_data,
                //     log_object.receive_headers
                // ));
                debug!("log insert data: {:?}", log_object);
                let request_id = match log_object.request_id {
                    Some(x) => x,
                    None => RequestId(Some(0))
                };

                let log_request = sqlx::query!(
                "INSERT INTO log (
                    parent_id,
                    request_id,
                    payment_id,
                    stage,
                    type,
                    name,
                    microtime,
                    result,
                    http_code,
                    data,
                    basis
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING id",
                log_object.parent_id,
                request_id.0,
                log_object.payment_id,
                log_object.stage,
                log_object.log_type.to_string(),
                log_object.name.to_string(),
                since_the_epoch_in_ms,
                log_object.result,
                log_object.http_code,
                log_object.data,
                log_object.basis
                )
                .fetch_one($pool);
                debug!("log insert await");

                let log_id = match log_request.await {
                    Ok(log_row) => {
                        debug!("log insert success: {:?}", log_row);
                        log_row.id
                    },
                    Err(e) => {
                        error!("db error while log_bg: {:?}", e);
                        0
                    }
                };

                debug!("log insert result: {:?}", log_id);

                log_id
            // }
        // )
        }
    }
}

macro_rules! query_with_log {
    ($pool:expr, $request_context:expr, $sql:expr, $($opt:expr),*) => {
        {
            // Create log
            let mut data = String::new();
            $(
                data.push_str(&format!("{:?}", $opt));
                data.push_str(", ");
            )*
            let log = LogModel {
                parent_id: Option::None,
                request_id: Some($request_context.request_id),
                payment_id: Option::None,
                stage: LogStage::Unknown.to_string(),
                log_type: LogType::DB,
                name: LogName::Unknown,
                result: Option::None,
                http_code: Option::None,
                data: data,
                basis: String::from($sql),
            };
            let log_id_fn = log_insert_db!(log, $pool);

            // Request
            let result;// = InnerResult::Ok( InnerResultElement {info: InnerResultInfo( String::from( InnerResultInfo::OK ) ), ..Default::default()} );

            let db_request = sqlx::query!(
                $sql,
                $(
                    $opt,
                )*
            )
            .fetch_one($pool)
            .await;

            match &db_request {
                Ok(row) => {
                    // debug!("record insert success: {:?}", row);
                    result = InnerResult::Ok(
                        InnerResultElement {
                            info: InnerResultInfo( String::from( InnerResultInfo::OK ) ),
                            detail: Some(String::from(&*format!("{:?}", row)))
                         }
                     );
                },
                Err(e) => {
                    error!("db error while insert: {:?}", e);
                    result = InnerResult::ErrorUnknown(
                        InnerResultElement {
                            info: InnerResultInfo(String::from(InnerResultInfo::ERROR_UNKNOWN)),
                            detail: Some(String::from(&*format!("{:?}", e)))
                        }
                    );
                }
            };

            // Create log
            let mut data = String::new();
            $(
                data.push_str(&format!("{:?}", $opt));
                data.push_str(", ");
            )*
            let log = LogModel {
                parent_id: Some(log_id_fn),
                request_id: Some($request_context.request_id),
                payment_id: Option::None,
                stage: LogStage::Unknown.to_string(),
                log_type: LogType::DB,
                name: LogName::Unknown,
                result: Some(OuterResult::get_code(&result).0),
                http_code: Option::None,
                data: format!("{:?}", result),
                basis: String::from(""),
            };
            log_insert_db!(log, $pool);

            db_request
        }
    }
}
