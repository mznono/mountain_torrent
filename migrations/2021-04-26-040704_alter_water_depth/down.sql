-- This file should undo anything in `up.sql`
ALTER TABLE water_depths DROP flow_value;
ALTER TABLE water_depths DROP half_hour;
ALTER TABLE water_depths DROP one_hour;
ALTER TABLE water_depths DROP one_half_hour;
ALTER TABLE water_depths DROP two_hour;
ALTER TABLE water_depths DROP three_hour;