use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{Cursor, Read};
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use vid2img::FileSource;

pub fn extract_frames_using_videotools(video_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Construct the ffmpeg command
    let output_pattern = "output-%04d.png";
    let status = Command::new("ffmpeg")
        .arg("-hwaccel")
        .arg("videotoolbox")
        .arg("-i")
        .arg(video_path)
        .arg("-vf")
        .arg("fps=1/10")
        .arg("-pix_fmt")
        .arg("rgb24")
        .arg(output_pattern)
        .status()?;

    if !status.success() {
        return Err("ffmpeg command failed".into());
    }

    // Collect the extracted frame paths
    let mut frame_paths = Vec::new();
    let frame_pattern = Path::new(output_pattern)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let frame_prefix = &frame_pattern[..frame_pattern.len() - 8]; // Assuming output pattern is output_%04d.png

    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("png") {
            if path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(frame_prefix)
            {
                frame_paths.push(path.to_str().unwrap().to_string());
            }
        }
    }

    Ok(frame_paths)
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
