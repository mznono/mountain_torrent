-- Your SQL goes here
CREATE TABLE water_depth
(
    id SERIAL PRIMARY KEY,
    device_id INTEGER NOT NULL references devices,
    value NUMERIC(6,2) NOT NULL DEFAULT 0,
    create_time TIMESTAMP(0) WITH TIME ZONE NOT NULL
)