pub mod core {
    pub mod domain {
        pub struct Country {
            pub name: String,
            pub indicator: String,
            pub value: f64,
            pub year: u64,
        }
    }
    pub mod ports {
        trait Country {}

        pub trait CountryRepository {
            fn growth_info(&self) -> crate::core::domain::Country;
            fn size(&self) -> usize;
            fn update_growth(&mut self);
            fn create_country_growth_info(&mut self);
            fn remove_country_growth_info(&mut self);
        }
    }

    pub mod services {
        /// get status of processing (LoadGrowthInformation)
        pub struct StatusProcessService {}
        /// get some growth unit information of growth database
        pub struct GrowthInformationService {}
        /// save all growth information from file
        pub struct LoadGrowthInformationService {}
        /// update some growth unit information
        pub struct UpdateGrowthInformationService {}
        /// remove country growth information
        pub struct RemoveGrowthInformationService {}
    }
}

pub mod adapters {
    pub mod memory {
        use crate::core::domain::Country;
        use crate::core::ports::CountryRepository;
        use std::collections::HashMap;

        pub struct CountryMemoryRepository {
            db: HashMap<String, Country>,
        }

        impl CountryMemoryRepository {
            fn new() -> Self {
                let mut db: HashMap<String, Country> = HashMap::new();
                CountryMemoryRepository { db }
            }
        }
        impl CountryRepository for CountryMemoryRepository {
            fn growth_info(&self) -> crate::core::domain::Country {
                todo!()
            }

            fn size(&self) -> usize {
                self.db.len()
            }

            fn update_growth(&mut self) {
                todo!()
            }

            fn create_country_growth_info(&mut self) {
                todo!()
            }

            fn remove_country_growth_info(&mut self) {
                todo!()
            }
        }
    }
    pub mod http {
        use actix_web::{web, App, HttpServer, Responder};

        pub mod controllers {

            pub async fn status_handler() -> impl super::Responder {
                unimplemented!()
            }

            pub async fn size_handler() -> impl super::Responder {
                unimplemented!()
            }

            pub async fn growth_information_handler() -> impl super::Responder {
                unimplemented!()
            }

            pub async fn save_growth_information_handler() -> impl super::Responder {
                unimplemented!()
            }

            pub async fn update_growth_information_handler() -> impl super::Responder {
                unimplemented!()
            }

            pub async fn delete_growth_information_handler() -> impl super::Responder {
                unimplemented!()
            }
        }

        pub struct Server {}

        use std::io;

        impl Server {
            pub fn new() -> Self {
                Server {}
            }
            pub async fn start(&self) -> io::Result<()> {
                HttpServer::new(|| {
                    App::new().service(
                        web::scope("/api/v1/growth/")
                            .route("status", web::get().to(controllers::status_handler))
                            .route("size", web::get().to(controllers::size_handler))
                            .route(
                                "{country}/{indicator}/{year}",
                                web::get().to(controllers::growth_information_handler),
                            )
                            .route(
                                "{country}/{indicator}/{year}",
                                web::put().to(controllers::update_growth_information_handler),
                            )
                            .route(
                                "{country}/{indicator}/{year}",
                                web::delete().to(controllers::delete_growth_information_handler),
                            )
                            .route(
                                "/",
                                web::post().to(controllers::save_growth_information_handler),
                            ),
                    )
                })
                .bind("0.0.0.0:3000")
                .unwrap()
                .run()
                .await
            }
        }
    }
}

pub mod config {}
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let new_server = adapters::http::Server::new();
    new_server.start().await?;
    Ok(())
}
