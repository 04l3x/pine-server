pub mod download;
pub mod upload;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use git2::transport::*;
use std::io::{Read, Write};

#[derive(Debug, Deserialize)]
struct Query {
	_service: String,
}

#[derive(Debug, Deserialize)]
struct Path {
	_username: String,
	_repository: String,
}

#[get("/{username}/{repository}.git/info/refs")]
async fn handshake(
	_req: HttpRequest,
	_query: web::Query<Query>,
	_path: web::Path<Path>,
) -> impl Responder {
	/*println!("Req: \n{:?}\n", req);
	println!("Query: \n{:?}\n", query);
	println!("Path: \n{:?}\n", path);

	/*match service.service {
		"git-upload-pack" => {},
		"git-receive-pack" => {},
		_ => {}
	}*/

	println!("{:?}", req);*/
	//TODO
	HttpResponse::Ok()
}

struct ActixTransport {}

impl SmartSubtransport for ActixTransport {
	fn action(
		&self,
		_url: &str,
		_action: Service,
	) -> Result<Box<dyn SmartSubtransportStream>, git2::Error> {
		todo!()
	}

	fn close(&self) -> Result<(), git2::Error> {
		todo!()
	}
}

impl Read for ActixTransport {
	fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
		todo!()
	}
}

impl Write for ActixTransport {
	fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
		todo!()
	}

	fn flush(&mut self) -> std::io::Result<()> {
		todo!()
	}
}

pub fn init(cfg: &mut web::ServiceConfig) {
	cfg.service(handshake);
	cfg.service(download::download);
	cfg.service(upload::upload);
}
