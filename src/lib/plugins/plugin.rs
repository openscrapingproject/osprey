pub struct Plugin {
    // could this ever be dynamic
    name: &'static str,
}

use config::Config;

use inflector::Inflector;
use std::env;
use std::mem::drop;

use log::info;

// use actix_web::HttpServer;

// use actix_web::{get, web, App, HttpServer, HttpResponse};
// use std::io::Result;

impl Plugin {
    pub fn new(name: &'static str) -> Plugin {
        Plugin { name: name }
    }

    // Sets up reading configuration from files and environment variables. Logs
    // and returns retreived configuration
    pub fn initialize(self) -> super::Settings {
        let mut s = Config::default();
        let file_name = &format!("osprey-{}", self.name.to_kebab_case());
        let env_name = &format!("OSPREY_{}", self.name.to_snake_case());
        s.merge(config::File::with_name(file_name).required(false))
            .unwrap()
            .merge(config::Environment::with_prefix(env_name))
            .unwrap();

        let key = "PORT";
        match env::var(key) {
            Ok(val) => drop(s.set("port", val).unwrap()),
            Err(_e) => {}
        }

        let out = s.try_into::<super::Settings>().unwrap();
        info!("Starting plugin {} with {:?}", self.name, out);
        out
    }

    // pub fn bind_server(self) -> Result<HttpServer<dyn std::any::Any, _, _,
    // _>> {     HttpServer::new(|| {
    //         App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    //     })
    //     .bind("127.0.0.1:8088")
    // }
}
