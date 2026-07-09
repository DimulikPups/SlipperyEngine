use std::path::Path;
use std::process::Stdio;
use log::{error, info, warn};
use super::{Stage, Context};

#[derive(Debug)]
pub struct Interpolate;

impl Interpolate {
    pub fn new() -> Self {
        Self
    }
}

impl Stage for Interpolate {
    fn execute(&self, context: &mut Context) {
        context.new_fps = context.original_fps * context.multiply as f32;
        match context.vfi_model.as_str() {
            "gfmss" => {
                let frames_path = Path::new(&context.current_path);
                let processing_dir = frames_path.parent().unwrap();
                let interpolated_dir = processing_dir.join("interpolated");
                let interpolated_dir_str = interpolated_dir.to_string_lossy().into_owned();

                let model_dir = "src/models/vfi/gfmss/train_log";

                let mut args: Vec<String> = vec![
                    format!("--frames={}", context.current_path),
                    format!("--model={}", model_dir),
                    format!("--output={}", interpolated_dir_str),
                    format!("--multi={}", context.multiply),
                    format!("--scale={}", context.scale),
                    "--gpu=auto".to_string(),
                ];
                if context.fp16 {
                    args.push("--fp16".to_string());
                }
                if context.union {
                    args.push("--union".to_string());
                    info!("union mode (gfmss model) enabled.")
                }

                info!("Running GFMSS interpolation on frames from: {}", context.current_path);
                let mut cmd = std::process::Command::new("src/python/python-packed/python.exe")
                    .arg("src/models/vfi/gfmss/inference_video.py")
                    .args(&args)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn().expect("failed to run gfmss model");
                let status = cmd.wait().unwrap();
                if status.success() {
                    info!("interpolation using gfmss model completed successfully.");
                    context.current_path = interpolated_dir_str;
                } else {
                    error!("interpolation process did not complete successfully");
                    std::process::exit(1);
                }
            }

            "rife" => {
                let frames_path = Path::new(&context.current_path);
                let processing_dir = frames_path.parent().unwrap();
                let interpolated_dir = processing_dir.join("interpolated");
                let interpolated_dir_str = interpolated_dir.to_string_lossy().into_owned();

                let model_type = if context.vfi_model_type.is_empty() {
                    warn!("No RIFE model type specified, defaulting to 4.26");
                    "4.26".to_string()
                } else {
                    context.vfi_model_type.clone()
                };

                let mut args: Vec<String> = vec![
                    format!("--frames={}", context.current_path),
                    format!("--output={}", interpolated_dir_str),
                    format!("--type={}", model_type),
                    format!("--multi={}", context.multiply),
                    format!("--scale={}", context.scale),
                    "--gpu=auto".to_string(),
                ];
                if context.fp16 {
                    args.push("--fp16".to_string());
                }

                info!("Running RIFE interpolation (type: {}) on frames from: {}", model_type, context.current_path);
                let mut cmd = std::process::Command::new("src/python/python-packed/python.exe")
                    .arg("src/models/vfi/rife/inference_video.py")
                    .args(&args)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn().expect("failed to run rife model");
                let status = cmd.wait().unwrap();
                if status.success() {
                    info!("interpolation using rife model (type: {}) completed successfully.", model_type);
                    context.current_path = interpolated_dir_str;
                } else {
                    error!("interpolation process did not complete successfully");
                    std::process::exit(1);
                }
            }

            _ => {
                error!("vmodel type is incorrect.");
                std::process::exit(1);
            }
        }
    }
}

impl Default for Interpolate {
    fn default() -> Self {
        Self::new()
    }
}
