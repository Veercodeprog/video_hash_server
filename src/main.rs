use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use image::open;
use serde::{Deserialize, Serialize};

mod dhash;
mod phash;
mod video;
pub use crate::dhash::compute_dhash;
pub use crate::phash::compute_phash;
pub use crate::video::{extract_frames, extract_frames_with_videotoolbox};
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
    // Extract frames from the video file provided in the URL
    match video::extract_frames(&query.video_url, 10) {
        Ok(frame_paths) => {
            let mut hashes = Vec::new();
            for frame_path in frame_paths {
                // Open the image file
                match open(&frame_path) {
                    Ok(img) => {
                        // Compute pHash for the image
                        let phash = compute_phash(&img);
                        hashes.push(phash);
                    }
                    Err(e) => eprintln!("Error opening image file {}: {}", frame_path, e),
                }
            }
            HttpResponse::Ok().json(hashes) // Return the list of pHashes
        }
        Err(e) => {
            eprintln!("Error processing video: {}", e);
            HttpResponse::BadRequest().body("Error processing video")
        }
    }
}
// API for dHash
// #[get("/dhash")]
// async fn dhash_api(query: web::Query<VideoUrl>) -> impl Responder {
//     let frame_data = video::extract_frame(&query.video_url).unwrap();
//     let dhash = dhash::compute_dhash(&frame_data);
//     HttpResponse::Ok().json(HashResponse { hash: dhash })
// }

// // API for Blake3 hash
// #[get("/blake3")]
// async fn blake3_api(query: web::Query<VideoUrl>) -> impl Responder {
//     let blake3_hash = blake3::compute_blake3(&query.video_url).unwrap();
//     HttpResponse::Ok().json(HashResponse { hash: blake3_hash })
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(phash_api)
        // .service(dhash_api)
        // .service(blake3_api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
