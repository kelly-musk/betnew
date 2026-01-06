pub mod api {
    pub mod handler {}
    pub mod middleware {}
    pub mod routes;
}

pub mod core {
    pub mod models {
        pub mod user;
    }
}

pub mod infastructure{
    pub mod cache{}
    pub mod db{}
}
