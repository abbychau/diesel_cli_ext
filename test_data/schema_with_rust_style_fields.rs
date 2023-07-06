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

// CREATE TABLE text_types (
//   `id` INT NOT NULL,
//   `tinytext` TINYTEXT NOT NULL,
//   `nullable_tinytext` TINYTEXT NULL,
//   `mediumtext` MEDIUMTEXT NOT NULL,
//   `nullableMediumtext` MEDIUMTEXT NULL,
//   `longtext` MEDIUMTEXT NOT NULL,
//   `nullableLongtext` MEDIUMTEXT NULL,
//   PRIMARY KEY (`id`));
table! {
    text_types (id) {
        id -> Integer,
        tinytext -> Tinytext,
        nullableTinytext -> Nullable<Tinytext>,
        mediumtext -> Mediumtext,
        nullableMediumtext -> Nullable<Mediumtext>,
        longtext -> Mediumtext,
        nullableLongtext -> Nullable<Mediumtext>,
    }
}

// CREATE TABLE numeric_types (
//   `id` INT NOT NULL,
//   `double` DOUBLE NOT NULL,
//   `nullableDouble` DOUBLE NULL,
//   `tinyint` TINYINT NOT NULL,
//   `nullableTinyint` TINYINT NULL,
//   `smallint` SMALLINT NOT NULL,
//   `nullableSmallint` SMALLINT NULL,
//   `bigint` BIGINT NOT NULL,
//   `nullable_bigint` BIGINT NULL,
//   PRIMARY KEY (`id`));
table! {
    numeric_types (id) {
        id -> Integer,
        double -> Double,
        nullableDouble -> Nullable<Double>,
        tinyint -> Tinyint,
        nullableTinyint -> Nullable<Tinyint>,
        smallint -> Smallint,
        nullableSmallint -> Nullable<Smallint>,
        bigint -> Bigint,
        nullableBigint -> Nullable<Bigint>,
    }
}

// CREATE TABLE unsigned_types (
//   `id` INT NOT NULL,
//   `unsigned_tinyint` TINYINT UNSIGNED NOT NULL,
//   `nullable_unsigned_tinyint` TINYINT UNSIGNED NULL,
//   `unsigned_smallint` SMALLINT UNSIGNED NOT NULL,
//   `nullable_unsigned_smallint` SMALLINT UNSIGNED NULL,
//   `bigint` BIGINT UNSIGNED NOT NULL,
//   `nullable_bigint` BIGINT UNSIGNED NULL,
//   PRIMARY KEY (`id`));
table! {
    unsigned_types (id) {
        id -> Integer,
        unsignedTinyint -> Unsigned<Tinyint>,
        nullableUnsignedTinyint -> Nullable<Unsigned<Tinyint>>,
        unsignedSmallint -> Unsigned<Smallint>,
        nullableUnsignedSmallint -> Nullable<Unsigned<Smallint>>,
        bigint -> Unsigned<Bigint>,
        nullableBigint -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    blob_types (id) {
        id -> Integer,
        blob -> Blob,
        nullableBlob -> Nullable<Blob>,
        tinyblob -> Tinyblob,
        nullableTinyblob -> Nullable<Tinyblob>,
        mediumblob -> Mediumblob,
        nullableMediumblob -> Nullable<Mediumblob>,
        longblob -> Mediumblob,
        nullableLongblob -> Nullable<Mediumblob>,
    }
}

table! {
    binary_and_chars (id) {
        id -> Integer,
        char -> Char,
        varchar -> Varchar,
        binary -> Binary,
        varbinary -> Varbinary,
    }
}

table! {
    uppercase_fields (id) {
        ID -> Integer,
        NAME -> Varchar,
        CREATED_AT -> Nullable<Datetime>,
        UPDATED_AT -> Nullable<Datetime>,
    }
}
