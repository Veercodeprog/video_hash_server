use image::{DynamicImage, GenericImageView};
use imagehash::DifferenceHash;

pub fn compute_dhash(image: &DynamicImage) -> String {
    // Create a DifferenceHash instance with default parameters
    let hasher = DifferenceHash::new();

    // Compute the dHash for the image
    let hash = hasher.hash(image);

    // Print and return the hex-encoded hash string
    println!("{}", hash.to_string());
    hash.to_string()
}
