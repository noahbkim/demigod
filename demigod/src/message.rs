pub mod csgo {
    pub mod message {
        pub mod cstrike15 {
            pub mod gc {
                include!(concat!(env!("OUT_DIR"), "/csgo.message.cstrike15.gc.rs"));
            }
            pub mod user {
                include!(concat!(env!("OUT_DIR"), "/csgo.message.cstrike15.user.rs"));
            }
        }
        pub mod engine {
            pub mod gc {
                include!(concat!(env!("OUT_DIR"), "/csgo.message.engine.gc.rs"));
            }
        }
        pub mod net {
            include!(concat!(env!("OUT_DIR"), "/csgo.message.net.rs"));
        }
        pub mod steam {
            include!(concat!(env!("OUT_DIR"), "/csgo.message.steam.rs"));
        }
    }
}
