pub mod my_schema {

    #[derive(Queryable, Debug, Identifiable)]
    #[primary_key(my_pk)]
    pub struct myTable {
        pub my_pk: i32,
        pub some_field: String,
    }

    #[derive(Queryable, Debug, Identifiable)]
    #[primary_key(my_pk)]
    pub struct myTable2 {
        pub my_pk: i32,
        pub some_field: String,
    }

}

pub mod my_other_schema {

    #[derive(Queryable, Debug, Identifiable)]
    #[primary_key(my_pk)]
    pub struct myTable {
        pub my_pk: i32,
        pub some_field: String,
    }

}

pub mod tenant {

    #[derive(Queryable, Debug, Identifiable)]
    pub struct Asd {
        pub id: i32,
        pub some_field: String,
    }

    #[derive(Queryable, Debug, Identifiable)]
    #[primary_key(my_pk)]
    pub struct myTable {
        pub my_pk: i32,
        pub some_field: String,
    }

}
