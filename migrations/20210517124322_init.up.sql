-- Add up migration script here
CREATE TABLE logs
(
    id              bigserial NOT NULL PRIMARY KEY,
    request_id      bigint,
    payment_id      bigint,
    stage           text,
    log_type        text,
    microtime_bgn   bigint,
    microtime_end   bigint,
    result          integer,
    http_code       integer,
    send_data       text,
    send_headers    text,
    receive_data    text,
    receive_headers text,
    create_at       timestamp with time zone default current_timestamp,
    update_at       timestamp with time zone default current_timestamp
)
