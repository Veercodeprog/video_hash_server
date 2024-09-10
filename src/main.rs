use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod dhash;
mod phash;
mod video;
pub use crate::dhash::compute_dhash;
pub use crate::phash::compute_phash;
pub use crate::video::extract_frame;
#[derive(Deserialize)]
struct VideoUrl {
    video_url: String,
}

#[derive(Serialize)]
struct HashResponse {
    hash: String,
}

// API for pHash
#[get("/phash")]
async fn phash_api(query: web::Query<VideoUrl>) -> impl Responder {
    let frame_data = video::extract_frame(&query.video_url).unwrap();
    let phash = phash::compute_phash(&frame_data);
    HttpResponse::Ok().json(HashResponse { hash: phash })
}

// API for dHash
#[get("/dhash")]
async fn dhash_api(query: web::Query<VideoUrl>) -> impl Responder {
    let frame_data = video::extract_frame(&query.video_url).unwrap();
    let dhash = dhash::compute_dhash(&frame_data);
    HttpResponse::Ok().json(HashResponse { hash: dhash })
}

// // API for Blake3 hash
// #[get("/blake3")]
// async fn blake3_api(query: web::Query<VideoUrl>) -> impl Responder {
//     let blake3_hash = blake3::compute_blake3(&query.video_url).unwrap();
//     HttpResponse::Ok().json(HashResponse { hash: blake3_hash })
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(phash_api).service(dhash_api)
        // .service(blake3_api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
