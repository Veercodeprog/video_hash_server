use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{Cursor, Read};
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use vid2img::FileSource;

pub fn extract_frames_with_videotoolbox(video_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let start_time = Instant::now(); // Start timing hashing/fingerprinting

    let output = Command::new("ffmpeg")
        .args(&[
            "-hwaccel",
            "videotoolbox",
            "-i",
            video_path,
            "-vf",
            "fps=1",
            "-f",
            "image2pipe",
            "-pix_fmt",
            "yuvj444p",
            "-",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("FFmpeg error: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    let frame_data = output.stdout;

    let elapsed_time = start_time.elapsed(); // Stop timing frame extraction

    println!("Frame extraction completed in {:?}", elapsed_time);

    Ok(frame_data)
}

// Function to extract frames from the video and save them as PNGs
pub fn extract_frames(video_path: &str, interval_sec: u64) -> Result<Vec<String>, Box<dyn Error>> {
    let file_path = Path::new(video_path);
    let frame_source = FileSource::new(file_path, (200, 200))?;
    let mut frame_count = 0;
    let mut frame_paths = Vec::new();

    // Calculate the number of frames to skip based on the interval
    let frame_interval = interval_sec as usize;

    for (index, frame) in frame_source.into_iter().enumerate() {
        if let Ok(Some(png_img_data)) = frame {
            // Save frame only if it matches the interval
            if index % frame_interval == 0 {
                let output_filename = format!("frame_{}.png", frame_count);
                let mut output_file = File::create(&output_filename)?;
                output_file.write_all(&png_img_data)?;
                frame_paths.push(output_filename);
                println!("Saved frame: frame_{}.png", frame_count);
                frame_count += 1;
            }
        }
    }

    Ok(frame_paths) // Return the list of saved frame file paths
}
