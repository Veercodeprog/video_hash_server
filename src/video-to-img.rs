use std::fs::File;
use std::io::Write;
use std::path::Path;
use vid2img::FileSource;

fn video_to_image() {
    let file_path = Path::new("video.mp4"); // Path to your video file

    let frame_source = FileSource::new(file_path, (200, 200)).unwrap();
    let mut frame_count = 0;

    for frame in frame_source.into_iter() {
        if let Ok(Some(png_img_data)) = frame {
            let output_filename = format!("frame_{}.png", frame_count);
            let mut output_file = File::create(output_filename).unwrap();
            output_file.write_all(&png_img_data).unwrap();
            println!("Saved frame: frame_{}.png", frame_count);
            frame_count += 1;
        }
    }
}
