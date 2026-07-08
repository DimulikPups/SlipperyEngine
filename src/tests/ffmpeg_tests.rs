#[cfg(test)]
mod ffmpeg_tests {
    use crate::ffmpeg;

    #[test]
    fn test_extract_video_fps() {
        let result = ffmpeg::get_video_fps("src/tests/videos/videotest.mp4");
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_video_command() {
        let images_dir = "src/tests/images_test";
        let cmd = ffmpeg::build_video_command(
            images_dir,
            "%04d.png",
            "30",
            "libx264",
            "yuv420p",
            "output.mp4",
        );

        let args: Vec<_> = cmd.get_args().collect();

        assert_eq!(cmd.get_program(), "ffmpeg");
        assert_eq!(cmd.get_current_dir(), Some(std::path::Path::new(images_dir)));
        assert_eq!(args, [
            "-r", "30",
            "-i", "%04d.png",
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            "-y", "output.mp4"
        ]);
    }
}
