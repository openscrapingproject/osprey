use osprey::plugins::Plugin;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use std::net::{SocketAddrV4, Ipv4Addr};
use serde::{Deserialize, Serialize};
use reqwest;

use async_trait::async_trait;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let p = Plugin::new("Requestor");

    let settings = p.initialize();
    let addr: Ipv4Addr = settings.address;
    // match settings.address.parse() {
    //     Ok(a)  => a,
    //     Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    // };
    let bindaddr = SocketAddrV4::new(addr, settings.port);

    HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
        .bind(bindaddr)?
        .run()
        .await
}


struct Requestor;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    version: String,
}

#[async_trait]
impl osprey::plugin::Requestor<Config, simple_error::SimpleError> for Requestor {
    async fn make_request(url: String) -> Result<http::Response<()>, simple_error::SimpleError> {
        let resp = reqwest::get(url).await?;
        eprintln!("resp = {:#?}", resp);
        Ok(resp)
    }
}