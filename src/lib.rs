mod api {
    mod handlers {
        pub mod auth;
        pub mod events;
        pub mod leaderboard;
        pub mod predictions;
    }
    pub mod middleware {}
    pub mod routes;
}

pub mod core {
    pub mod models {
        pub mod user;
        pub mod Predictions;
    }
    pub mod services {
        pub mod scoring;
    }
}

mod infrastructure {
    pub mod cache {}
    pub mod db {}
}
