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

                let log_request = sqlx::query!("
                INSERT INTO log (
                    request_id,
                    payment_id,
                    stage,
                    type,
                    name,
                    microtime_bgn,
                    microtime_end,
                    -- result,
                    -- http_code,
                    in_data,
                    in_basis
                    -- out_data,
                    -- out_basis
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                RETURNING id
                ",
                request_id.0,
                log_object.payment_id,
                log_object.stage,
                log_object.log_type.to_string(),
                log_object.name.to_string(),
                since_the_epoch_in_ms,
                0,//log_object.microtime_end,
                // log_object.result,
                // log_object.http_code,
                log_object.in_data,
                log_object.in_basis,
                // log_object.out_data,
                // log_object.out_basis
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

macro_rules! log_update_db {
    ($log_object:expr, $pool:expr, $log_id:expr) => {
        {
        // block_on(
        //     async {
                let log_object = $log_object;

                let since_the_epoch = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                let since_the_epoch_in_ms = since_the_epoch.as_secs() as i64 * 1000 +
                    since_the_epoch.subsec_nanos() as i64 / 1_000_000;
                let time_now = Local::now();

                debug!("log update data: {:?}", log_object);

                let log_request = sqlx::query!("
                UPDATE log SET (
                    -- request_id,
                    payment_id,
                    -- stage,
                    -- type,
                    -- name,
                    -- microtime_bgn,
                    microtime_end,
                    result,
                    http_code,
                    -- in_data,
                    -- in_basis,
                    out_data,
                    out_basis,
                    update_at
                )
                = ($1, $2, $3, $4, $5, $6, $7)
                WHERE id = $8
                RETURNING id
                ",
                // log_object.request_id,
                log_object.payment_id,
                // log_object.stage,
                // log_object.log_type.to_string(),
                // log_object.name.to_string(),
                // log_object.microtime_bgn,
                since_the_epoch_in_ms,
                log_object.result,
                log_object.http_code,
                // log_object.in_data,
                // log_object.in_basis,
                log_object.out_data,
                log_object.out_basis,
                time_now,

                $log_id
                )
                .fetch_one($pool);
                debug!("log update await");

                let log_id = match log_request.await {
                    Ok(log_row) => {
                        debug!("log update success: {:?}", log_row);
                        log_row.id
                    },
                    Err(e) => {
                        error!("db error while log update: {:?}", e);
                        0
                    }
                };

                debug!("log update result: {:?}", log_id);
            // }
        // )
        }
    }
}
