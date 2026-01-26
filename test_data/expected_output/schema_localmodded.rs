pub mod my_schema {
    use super::*;

    #[derive(Queryable, Debug, Identifiable)]
    #[diesel(primary_key(my_pk))]
    pub struct MyTable {
        pub my_pk: i32,
        pub some_field: String,
    }

    #[derive(Queryable, Debug, Identifiable)]
    #[diesel(primary_key(my_pk))]
    pub struct MyTable2 {
        pub my_pk: i32,
        pub some_field: String,
    }

}

pub mod my_other_schema {
    use super::*;

    #[derive(Queryable, Debug, Identifiable)]
    #[diesel(primary_key(my_pk))]
    pub struct MyTable {
        pub my_pk: i32,
        pub some_field: String,
    }

}

pub mod tenant {
    use super::*;

    #[derive(Queryable, Debug, Identifiable)]
    pub struct Asd {
        pub id: i32,
        pub some_field: String,
    }

    #[derive(Queryable, Debug, Identifiable)]
    #[diesel(primary_key(my_pk))]
    pub struct MyTable {
        pub my_pk: i32,
        pub some_field: String,
    }

}
