mod git;

use crate::auth;
use crate::graphql;
use crate::utils::{config::Config, database};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

///the server serve three things
///the graphql api that is cosumed by the frontend client
///post and get routes for repositoriess, to enable clone, push, and pull git operations
pub struct Server;

impl Server {
	pub async fn start() -> std::io::Result<()> {
		let schema = graphql::build_schema().await;

		let pool = database::default_pool().await;

		HttpServer::new(move || {
			let cors = Cors::permissive();

			App::new()
				.wrap(cors)
				.wrap(Logger::default())
				.app_data(web::Data::new(schema.clone()))
				.service(web::resource("/api").guard(guard::Post()).to(index))
				.service(
					web::resource("/api")
						.guard(guard::Get())
						.to(index_playground),
				)
				.service(
					web::scope("/git")
						.app_data(web::Data::new(pool.clone()))
						.configure(git::init),
				)
		})
		.bind(Config::url())?
		.run()
		.await
	}
}

async fn index(
	schema: web::Data<graphql::Schema>,
	req: HttpRequest,
	gql_request: Request,
) -> Response {
	let token = req.headers().get("Token").and_then(|value| {
		value
			.to_str()
			.map(|s| auth::session::Token(s.to_string()))
			.ok()
	});

	let mut request = gql_request.into_inner();

	if let Some(token) = token {
		request = request.data(token);
	}

	schema.execute(request).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
	let source = playground_source(GraphQLPlaygroundConfig::new("/api").subscription_endpoint("/"));
	Ok(HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(source))
}
