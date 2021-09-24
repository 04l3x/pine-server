use actix_web::{get, guard, post, web, App, Error, HttpResponse, HttpServer, Responder, Result};
use std::{env, path};

use actix_files as fs;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

use crate::graphql;

pub struct Server;

impl Server {
    pub async fn start() -> std::io::Result<()> {
        let url = format!(
            "{}:{}",
            env::var("HOST").expect("host error"),
            env::var("PORT").expect("port error")
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

/*#[get("/git/{namespace}/{repository}.git")]
async fn git_get() -> impl Responder {
    let git_workspace_dir = "/git/storage";
    let repo_name = format!("a-cool-name-for-a-cool-repo{}",".git");
    let url = path::PathBuf::from(
        format!("{}/{}", git_workspace_dir, repo_name)
    );
    println!("{:?}", url);
    HttpResponse::Ok().body("")
}*/
