use std::path::{Path, PathBuf};

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
        let output_path: PathBuf = if context.output_path.is_empty() {
            let frames_path = Path::new(&context.current_path);
            let processing_dir = frames_path.parent().unwrap();
            let original_video_dir = processing_dir.parent().unwrap();
            let processing_name = processing_dir.file_name().unwrap().to_string_lossy();
            if let Some(video_name) = processing_name.strip_suffix("_processing") {
                let video_path = Path::new(video_name);
                let stem = video_path.file_stem().unwrap().to_string_lossy();
                let ext = video_path.extension().map(|e| e.to_string_lossy()).unwrap_or_default();
                original_video_dir.join(format!("{}_merged.{}", stem, ext))
            } else {
                original_video_dir.join(format!("{}_merged", processing_name))
            }
        } else {
            PathBuf::from(&context.output_path)
        };

        if context.current_path.is_empty() {
            log::warn!("Skipping encode stage: no input path set");
            return;
        }

        let fps_rounded = context.new_fps.round() as u32;
        if let Err(e) = ffmpeg::images_to_video_command(
            context.current_path.as_str(),
            output_path.to_string_lossy().as_ref(),
            fps_rounded,
            &context.codec,
            &context.codec_args,
        ) {
            log::error!("Encoding failed: {:?}", e);
            return;
        }
        context.current_path = output_path.to_string_lossy().into_owned();
    }
}

impl Default for Encode {
    fn default() -> Self {
        Self::new()
    }
}
