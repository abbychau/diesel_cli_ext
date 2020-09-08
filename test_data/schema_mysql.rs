table! {
    units_of_measure (id) {
        id -> Unsigned<Integer>,
        volumetricUnitId -> Nullable<Unsigned<Integer>>,
        weightUnitId -> Nullable<Unsigned<Integer>>,
        number -> Nullable<Decimal>,
        createdAt -> Nullable<Datetime>,
        updatedAt -> Nullable<Datetime>,
    }
}

table! {
    something_types (id) {
        id -> Unsigned<Integer>,
        value -> Varchar,
        name -> Varchar,
        active -> Bit,
        createdAt -> Nullable<Datetime>,
        updatedAt -> Nullable<Datetime>,
        inbound -> Bool,
    }
}

table! {
    something (id) {
        id -> Unsigned<Integer>,
        somethingId -> Nullable<Unsigned<Integer>>,
        somethingInOunces -> Decimal,
        total -> Unsigned<Decimal>,
        createdAt -> Nullable<Datetime>,
        updatedAt -> Nullable<Datetime>,
    }
}