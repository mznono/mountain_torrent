-- This file should undo anything in `up.sql`
DROP TABLE rainfalls;
-- select device_id, sum(value),half_hour from rainfalls group by (half_hour,device_id) order by (half_hour,device_id);