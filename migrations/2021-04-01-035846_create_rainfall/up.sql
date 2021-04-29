-- Your SQL goes here
CREATE TABLE rainfalls
(
    id SERIAL PRIMARY KEY,
    device_id INTEGER NOT NULL references devices,
    value NUMERIC(6,2) NOT NULL DEFAULT 0,
    create_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    half_hour BIGINT NOT NULL,
    one_hour BIGINT NOT NULL,
    one_half_hour BIGINT NOT NULL,
    two_hour BIGINT NOT NULL,
    three_hour BIGINT NOT NULL
)