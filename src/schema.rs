table! {
    calculations (id) {
        id -> Int4,
        device_id -> Int4,
        storage -> Numeric,
        wi -> Numeric,
        quantity -> Numeric,
        create_time -> Timestamptz,
        half_hour -> Int8,
        one_hour -> Int8,
        one_half_hour -> Int8,
        two_hour -> Int8,
        three_hour -> Int8,
    }
}

table! {
    devices (id) {
        id -> Int4,
        region -> Varchar,
        name -> Varchar,
        device_id -> Varchar,
        dike_height -> Numeric,
        half_hour_design -> Numeric,
        one_hour_design -> Numeric,
        one_half_hour_design -> Numeric,
        two_hour_design -> Numeric,
        three_design -> Numeric,
        stream_width -> Nullable<Numeric>,
        rainfall_area -> Nullable<Numeric>,
    }
}

table! {
    rainfalls (id) {
        id -> Int4,
        device_id -> Int4,
        value -> Numeric,
        create_time -> Timestamptz,
        half_hour -> Int8,
        one_hour -> Int8,
        one_half_hour -> Int8,
        two_hour -> Int8,
        three_hour -> Int8,
    }
}

table! {
    water_depths (id) {
        id -> Int4,
        device_id -> Int4,
        value -> Numeric,
        create_time -> Timestamptz,
        flow_value -> Nullable<Numeric>,
        half_hour -> Nullable<Int8>,
        one_hour -> Nullable<Int8>,
        one_half_hour -> Nullable<Int8>,
        two_hour -> Nullable<Int8>,
        three_hour -> Nullable<Int8>,
    }
}

joinable!(calculations -> devices (device_id));
joinable!(rainfalls -> devices (device_id));
joinable!(water_depths -> devices (device_id));

allow_tables_to_appear_in_same_query!(
    calculations,
    devices,
    rainfalls,
    water_depths,
);
