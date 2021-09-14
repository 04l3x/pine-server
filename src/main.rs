mod config;
mod database;
mod models;
mod graphql;

use actix_files as fs;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_actix_web::{Request, Response};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
	config::Config::run().await;
    env_logger::init();

    let url = &format!(
		"{}:{}", 
		env::var("HOST").expect("error NOT HOST VARIABLE"), 
		env::var("PORT").expect("error NOT HOST VARIABLE")
	);

	let schema = graphql::build_schema().await;

	let client_dir = env::var("CLIENT_PATH").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/api").guard(guard::Post()).to(index))
            .service(
                web::resource("/api")
                    .guard(guard::Get())
                    .to(index_playground),
            )
            .service(fs::Files::new("/", client_dir.as_str()).index_file("index.html"))
    })
    .bind(url)?
    .run()
    .await
}

async fn index(schema: web::Data<graphql::Schema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/api").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

