-- Add down migration script here
DROP SEQUENCE api_request_id_seq;

DROP TABLE log;

DROP TABLE record;
