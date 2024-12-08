// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct SitesStartPeriodEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct SitesStatusEnum;
}

diesel::table! {
    clients (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 20]
        phone_number -> Varchar,
    }
}

diesel::table! {
    site_managers (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    site_vehicles (site_id, vehicle_id) {
        site_id -> Unsigned<Bigint>,
        vehicle_id -> Unsigned<Bigint>,
    }
}

diesel::table! {
    site_workers (site_id, worker_id) {
        site_id -> Unsigned<Bigint>,
        worker_id -> Unsigned<Bigint>,
    }
}

diesel::table! {
    use diesel::sql_types::{Bigint, Float, Integer, Text, Timestamp, Unsigned, Varchar};
    use super::sql_types::SitesStartPeriodEnum;
    use super::sql_types::SitesStatusEnum;

    sites (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        purpose -> Text,
        latitude -> Float,
        longitude -> Float,
        start_day -> Timestamp,
        duration_half_day -> Integer,
        #[max_length = 9]
        start_period -> SitesStartPeriodEnum,
        #[max_length = 11]
        status -> SitesStatusEnum,
        site_manager_id -> Unsigned<Bigint>,
        client_id -> Unsigned<Bigint>,
    }
}

diesel::table! {
    sites_global_managers (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Text,
        role -> Nullable<Text>,
    }
}

diesel::table! {
    vehicles (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
        reserved_dates -> Longtext,
    }
}

diesel::table! {
    workers (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(site_vehicles -> sites (site_id));
diesel::joinable!(site_vehicles -> vehicles (vehicle_id));
diesel::joinable!(site_workers -> sites (site_id));
diesel::joinable!(site_workers -> workers (worker_id));
diesel::joinable!(sites -> clients (client_id));
diesel::joinable!(sites -> site_managers (site_manager_id));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    site_managers,
    site_vehicles,
    site_workers,
    sites,
    sites_global_managers,
    users,
    vehicles,
    workers,
);
