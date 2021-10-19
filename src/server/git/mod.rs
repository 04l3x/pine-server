pub mod download;
pub mod upload;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use git2::transport;

#[derive(Debug, Deserialize)]
struct Service{
	service: String
}

#[get("/{username}/{repository}.git/info/refs")]
async fn handshake(req: HttpRequest, service: web::Query<Service>) -> impl Responder {
	println!("{:?}", service);
	/*match service.service {
		"git-upload-pack" => {},
		"git-receive-pack" => {},
		_ => {}
	}*/
	println!("{:?}", req);
	HttpResponse::Ok()
}


pub fn init(cfg: &mut web::ServiceConfig) {
	cfg.service(handshake);
	cfg.service(download::download);
	cfg.service(upload::upload);
}
