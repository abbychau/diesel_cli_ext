pub mod my_schema {
    table! {
        my_schema.my_table (my_pk) {
            my_pk -> Int4,
            some_field -> Text,
        }
    }
    table! {
        my_schema.my_table2 (my_pk) {
            my_pk -> Int4,
            some_field -> Text,
        }
    }
}

pub mod my_other_schema {
    table! {
        my_other_schema.my_table (my_pk) {
            my_pk -> Int4,
            some_field -> Text,
        }
    }
}

pub mod tenant {
    table! {
        use crate::diesel_types::org::*;

        asd (id) {
            id -> Int4,
            some_field -> Text,
        }
    }

    table! {
        my_other_schema.my_table (my_pk) {
            my_pk -> Int4,
            some_field -> Text,
        }
    }
}