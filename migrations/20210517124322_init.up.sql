-- Add up migration script here
CREATE SEQUENCE api_request_id_seq
    START WITH 1
    INCREMENT BY 1
    MINVALUE 1
    NO MAXVALUE
    CACHE 1;

CREATE TABLE log
(
    id              bigserial NOT NULL PRIMARY KEY,
    request_id      bigint,
    payment_id      bigint,
    stage           text,
    type            text,
    name            text,
    microtime_bgn   bigint,
    microtime_end   bigint,
    result          integer,
    http_code       integer,
    in_data         text,
    in_basis        text,
    out_data        text,
    out_basis       text,
    create_at       timestamp with time zone default current_timestamp,
    update_at       timestamp with time zone default current_timestamp
);

CREATE TABLE record
(
    id              bigserial NOT NULL PRIMARY KEY,
    outer_id        bigint,
    stage           text,
    sum             integer,
    create_at       timestamp with time zone default current_timestamp,
    update_at       timestamp with time zone default current_timestamp
);
