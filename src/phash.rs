use image::{DynamicImage, GenericImageView};
use imagehash::PerceptualHash;

pub fn compute_phash(image: &DynamicImage) -> String {
    let hasher = PerceptualHash::new();
    let hash = hasher.hash(image);
    println!("{}", hash.to_string());
    return hash.to_string(); // Returns the hex-encoded hash string
}
