-- Your SQL goes here
ALTER TABLE water_depths ADD flow_value NUMERIC(6,2);
ALTER TABLE water_depths ADD half_hour BIGINT;
ALTER TABLE water_depths ADD one_hour BIGINT;
ALTER TABLE water_depths ADD one_half_hour BIGINT;
ALTER TABLE water_depths ADD two_hour BIGINT;
ALTER TABLE water_depths ADD three_hour BIGINT;