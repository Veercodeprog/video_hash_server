use std::fs::File;
use std::io::Read;

pub fn compute_blake3(video_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(video_path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);
    Ok(hasher.finalize().to_hex().to_string())
}
