#[macro_export]
macro_rules! log_db {
    ($log_object:expr, $pool:expr) => {
        block_on(
            async {
                let log_object = $log_object;

                // why not work?
                // debug!("log insert data: {:?}", (
                //     log_object.request_id,
                //     log_object.payment_id,
                //     log_object.stage,
                //     log_object.log_type,
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

                let log_request = sqlx::query!("
                INSERT INTO logs (
                    request_id,
                    payment_id,
                    stage,
                    log_type,
                    microtime_bgn,
                    microtime_end,
                    result,
                    http_code,
                    send_data,
                    send_headers,
                    receive_data,
                    receive_headers
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                RETURNING id
                ",
                log_object.request_id,
                log_object.payment_id,
                log_object.stage,
                log_object.log_type,
                log_object.microtime_bgn,
                log_object.microtime_end,
                log_object.result,
                log_object.http_code,
                log_object.send_data,
                log_object.send_headers,
                log_object.receive_data,
                log_object.receive_headers
                )
                .fetch_one(&$pool);

                let log_id = match log_request.await {
                    Ok(log_row) => {
                        log_row.id
                    },
                    Err(e) => {
                        error!("db error while log_bg: {:?}", e);
                        0
                    }
                };

                debug!("log insert result: {:?}", log_id);
            }
        )
    }
}
