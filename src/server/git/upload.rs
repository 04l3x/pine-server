use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/{username}/{repository}.git/info/refs?service=git-receive-pack")]
async fn handshake(req: HttpRequest) -> impl Responder {
	println!("{:?}", req);
	HttpResponse::Ok()
}

#[post("/{username}/{repository}.git/git-receive-pack")]
async fn upload(req: HttpRequest) -> impl Responder {
	println!("{:?}", req);
	HttpResponse::Ok()
}

pub fn init(cfg: &mut web::ServiceConfig) {
	cfg.service(handshake);
	cfg.service(upload);
}
