// @generated automatically by Diesel CLI.

pub mod app {
    diesel::table! {
        app.maintenance_logs (id) {
            id -> Int4,
            motorcycle_id -> Int4,
            activity_date -> Date,
            #[max_length = 100]
            activity_type -> Varchar,
            odometer_reading -> Nullable<Int4>,
            description -> Nullable<Text>,
            logged_at -> Timestamptz,
        }
    }

    diesel::table! {
        app.motorcycles (id) {
            id -> Int4,
            make -> Text,
            model -> Text,
            year -> Nullable<Int4>,
            license_plate -> Text,
            displacement -> Nullable<Int4>,
            created_at -> Timestamptz,
        }
    }

    diesel::joinable!(maintenance_logs -> motorcycles (motorcycle_id));

    diesel::allow_tables_to_appear_in_same_query!(
        maintenance_logs,
        motorcycles,
    );
}
