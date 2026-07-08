use crate::ffmpeg;
use super::{Stage, Context};

#[derive(Debug)]
pub struct Encode;

impl Encode {
    pub fn new() -> Self {
        Self
    }
}

impl Stage for Encode {
    fn execute(&self, context: &mut Context) {
        ffmpeg::images_to_video_command(
            context.current_path.as_str(),
            context.output_path.as_str(),
            context.new_fps as u32).unwrap();
        context.current_path = context.output_path.clone();
    }
}

impl Default for Encode {
    fn default() -> Self {
        Self::new()
    }
}
