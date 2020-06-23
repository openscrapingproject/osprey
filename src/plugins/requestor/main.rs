use osprey::plugins::Plugin;

use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::{SocketAddrV4, Ipv4Addr};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let p = Plugin::new("Requestor");

    let settings = p.initialize();
    let addr: Ipv4Addr = settings.address;
    let bindaddr = SocketAddrV4::new(addr, settings.port);

    HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
        .bind(bindaddr)?
        .run()
        .await
}