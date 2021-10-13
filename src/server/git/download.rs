use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/{username}/{repository}.git/info/refs?service=git-upload-pack")]
async fn handshake(req: HttpRequest) -> impl Responder {
	println!("{:?}", req);
	HttpResponse::Ok()
}

#[post("/{username}/{repository}.git/git-upload-pack")]
async fn download(req: HttpRequest) -> impl Responder {
	println!("{:?}", req);
	HttpResponse::Ok()
}

pub fn init(cfg: &mut web::ServiceConfig) {
	cfg.service(handshake);
	cfg.service(download);
}
