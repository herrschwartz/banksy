#[macro_use]
extern crate tera;

#[macro_use]
extern crate actix_web;

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer, Result,};
use actix_files as fs;

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: i32,
    amt: i32,
}

#[get("/favicon")]
fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

// store tera template in application state
fn index(
    tmpl: web::Data<tera::Tera>,
    _query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let s = {
        let mut ctx = tera::Context::new();

        let ids = vec![Item{id: 5113, amt: 1}, Item{id: 16204, amt: 20}, Item{id: 16393, amt: 1}, Item{id: 19019, amt: 1},
            Item{id: 14551, amt: 1}, Item{id: 14552, amt: 1}, Item{id: 16223, amt: 1}];

        ctx.insert("items", &ids);
        tmpl.render("index.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() -> std::io::Result<()> {
    println!("runnin on port 80");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera =
            compile_templates!("/templates/**/*");

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default()) // enable logger
            .service(favicon)
            .service(web::resource("/").route(web::get().to(index)))
            .service(fs::Files::new("/static", "static").show_files_listing())
    })
    .bind("0.0.0.0:80")?
    .run()
}