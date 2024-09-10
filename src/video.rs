use ffmpeg_next as ffmpeg;

pub fn extract_frame(video_path: &str) -> Result<Vec<u8>, ffmpeg::Error> {
    ffmpeg::init().unwrap();

    // Open video file
    let mut ictx = ffmpeg::format::input(&video_path)?;
    let video_stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = video_stream.index();
    let decoder = ffmpeg::codec::context::Context::from_parameters(video_stream.parameters())?
        .decoder()
        .video()?;

    let mut frame = ffmpeg::frame::Video::empty();
    let mut scaler = ffmpeg::software::scaling::context::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg::software::scaling::flag::BILINEAR,
    )?;

    let mut output = Vec::new();
    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.decode(&packet, &mut frame)?;
            let mut rgb_frame = ffmpeg::frame::Video::empty();
            scaler.run(&frame, &mut rgb_frame)?;
            // Do something with the frame, e.g., save it to the `output`
            output.extend_from_slice(rgb_frame.data(0));
            break;
        }
    }
    Ok(output)
}
