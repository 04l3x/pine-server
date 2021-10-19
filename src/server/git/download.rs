use actix_web::{post, HttpRequest, HttpResponse, Responder};

#[post("/{username}/{repository}.git/git-upload-pack")]
pub async fn download(req: HttpRequest) -> impl Responder {
	println!("{:?}", req);
	HttpResponse::Ok()
}

