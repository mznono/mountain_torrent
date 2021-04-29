-- Your SQL goes here
CREATE TABLE devices (
    id SERIAL PRIMARY KEY,
    region VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    device_id VARCHAR NOT NULL,
    dike_height NUMERIC(6,2) NOT NULL DEFAULT 0,
    half_hour_design NUMERIC(6,2) NOT NULL DEFAULT 9999.99,
    one_hour_design  NUMERIC(6,2) NOT NULL DEFAULT 9999.99,
    one_half_hour_design NUMERIC(6,2) NOT NULL DEFAULT 9999.99,
    two_hour_design NUMERIC(6,2) NOT NULL DEFAULT 9999.99,
    three_design NUMERIC(6,2) NOT NULL DEFAULT 9999.99
)