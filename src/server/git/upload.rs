use actix_web::{post, HttpRequest, HttpResponse, Responder};

#[post("/{username}/{repository}.git/git-receive-pack")]
pub async fn upload(req: HttpRequest) -> impl Responder {
	println!("{:?}", req);
	HttpResponse::Ok()
}

