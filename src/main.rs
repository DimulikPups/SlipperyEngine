pub mod fs;
pub mod ffmpeg;
pub mod engine;
pub mod pipeline;
pub mod python;

#[cfg(test)]
pub mod tests;

use std::env;
use log::{error, info};
use crate::pipeline::{Builder, Context};

fn init_logger() {
    let env = pretty_env_logger::env_logger::Env::default()
        .filter_or("RUST_LOG", "debug");

    pretty_env_logger::formatted_builder()
        .parse_env(env)
        .init();
}

#[tokio::main]
async fn main() {
    init_logger();
    info!("initialized. collecting arguments.");
    let args: Vec<String> = env::args().collect();
    let parsed_args = parse_args(&args);
    info!("parsed arguments: {:?}", args);
    info!("checking python environment...");
    let python_status = python::ensure_python_ready().await;
    if !python_status.is_ok() {
        error!("python not ready. aborting. try to delete <<SRC/PYTHON>> folder and launch script again.");
        return
    }
    match parsed_args {
        Ok(mut parsed_args) => {
            let media_type: &str = validate_media_type(parsed_args.input_file.as_str()); // video/images
            match parsed_args.processing_type {
                0 => {
                    let (pipeline, context) = Builder::new()
                        .with_encoding(true)
                        .with_codec(&parsed_args.codec)
                        .with_codec_args(std::mem::take(&mut parsed_args.codec_args))
                        .build();

                    let mut context = context
                        .with_new_fps(parsed_args.fps);
                    to_images_path(media_type, &mut context, &parsed_args);

                    let completed_context: Context = engine::execute(&pipeline, context);
                    info!("execution completed successfully. final path: {:?}", completed_context.current_path);
                },
                1 => {
                    let (pipeline, context) = Builder::new()
                        .with_interpolation()
                        .with_vfi_model(&parsed_args.vfi_model)
                        .with_scale(parsed_args.scale)
                        .with_union(parsed_args.union)
                        .with_fp16(parsed_args.vfi_fp16)
                        .with_encoding(true)
                        .with_codec(&parsed_args.codec)
                        .with_codec_args(std::mem::take(&mut parsed_args.codec_args))
                        .build();

                    let mut context = context
                        .with_original_fps(parsed_args.fps)
                        .with_multiply(parsed_args.multi);
                    to_images_path(media_type, &mut context, &parsed_args);

                    let completed_context: Context = engine::execute(&pipeline, context);
                    info!("execution completed successfully. final path: {:?}", completed_context.current_path);
                }
                _ => {
                    error!("invalid type specified.");
                    return
                }
            }
        },
        Err(err) => error!("{}", err),
    }
}

#[derive(Debug)]
pub struct ParsedArgs {
    pub input_file: String,
    pub output_file: String,
    pub processing_type: i8,
    pub codec: String,
    pub codec_args: Vec<String>,
    pub vfi_model: String,
    pub fps: f32,
    pub scale: f32,
    pub multi: i8,
    pub vfi_fp16: bool,
    pub union: bool,
}

impl<'a> IntoIterator for &'a ParsedArgs {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut args = vec![
            "-i".to_string(), self.input_file.clone(),
            "-o".to_string(), self.output_file.clone(),
            "-vmodel".to_string(), self.vfi_model.clone(),
            "-type".to_string(), self.processing_type.to_string(),
            "-fps".to_string(), self.fps.to_string(),
            "-scale".to_string(), self.scale.to_string(),
            "-multi".to_string(), self.multi.to_string(),
            "-fp16".to_string(), self.vfi_fp16.to_string(),
            "-union".to_string(), self.union.to_string(),
        ];
        if !self.codec.is_empty() {
            args.push("-codec".to_string());
            args.push(self.codec.clone());
        }
        /*
        libx264 (CPU)
        h264_nvenc (NVIDIA GPU)
        h264_amf (AMD GPU)
        h264_qsv (Intel QuickSync)
        hevc_nvenc (NVIDIA GPU)
        hevc_amf (AMD GPU)
        hevc_qsv (Intel QuickSync)
        prores_ks (CPU)

        -i input.mp4 -o output.mp4 -vmodel rife -type 1 -fps 60 \
        -codec h264_nvenc -codec-arg preset p7 -codec-arg crf 18

        */
        for arg in self.codec_args.iter().cloned() {
            args.push("-codec-arg".to_string());
            args.push(arg);
        }
        args.into_iter()
    }
}

impl Default for ParsedArgs {
    fn default() -> Self {
        Self {
            input_file: String::new(),
            output_file: "".to_string(),
            vfi_model: String::new(),
            processing_type: 0,
            fps: 0.0,
            scale: 1.0,
            multi: 2,
            vfi_fp16: false,
            union: false,
            codec: "libx264".to_string(),
            codec_args: Vec::new(),
        }
    }
}

pub fn parse_args(args: &[String]) -> Result<ParsedArgs, String> {
    if args.len() <= 1 {
        return Err("No arguments provided.".to_string());
    }

    let mut parsed_args: ParsedArgs = Default::default();
    let mut i = 1;
    while i < args.len() {
        let value = args
            .get(i + 1)
            .ok_or_else(|| format!("Missing value for argument: {}", args[i]))?;

        match args[i].as_str() {
            "-i" => parsed_args.input_file = value.clone(),
            "-o" => parsed_args.output_file = value.clone(),
            "-vmodel" => parsed_args.vfi_model = value.clone(),
            "-type" => parsed_args.processing_type = value.parse::<i8>().map_err(|e| e.to_string())?,
            "-fps" => parsed_args.fps = value.parse::<f32>().map_err(|_| "Failed to parse fps argument".to_string())?,
            "-scale" => parsed_args.scale = value.parse::<f32>().map_err(|_| "Failed to parse scale argument".to_string())?,
            "-multi" => parsed_args.multi = value.parse::<i8>().map_err(|_| "Failed to parse multi argument".to_string())?,
            "-fp16" => parsed_args.vfi_fp16 = value.parse::<bool>().map_err(|_| "Failed to parse fp16 argument".to_string())?,
            "-union" => parsed_args.union = value.parse::<bool>().map_err(|_| "Failed to parse union argument".to_string())?,
            "-codec" => parsed_args.codec = value.clone(),
            "-codec-arg" => {
                if let Some((key, val)) = value.split_once('=') {
                    parsed_args.codec_args.push(format!("-{}", key));
                    parsed_args.codec_args.push(val.to_string());
                } else {
                    parsed_args.codec_args.push(value.clone());
                }
            }
            _ => return Err(format!("invalid argument: {}", args[i])),
        }

        i += 2;
    }

    if parsed_args.input_file.is_empty() {
        return Err("no input file provided.".to_string());
    }
    if parsed_args.output_file.is_empty() {
        return Err("no output folder provided.".to_string());
    }
    if parsed_args.vfi_model.is_empty() {
        return Err("no VFI model type provided.".to_string());
    }
    if parsed_args.fps <= 0.0 {
        return Err("no valid fps provided.".to_string());
    }

    Ok(parsed_args)
}

fn to_images_path(media_type: &str, context: &mut Context, parsed_args: &ParsedArgs) {
    match media_type {
        "video" => {
            context.current_path = ffmpeg::extract_video_to_images_command(
                parsed_args.input_file.as_str()
            ).unwrap();
        }
        "images" => {
            context.current_path = parsed_args.input_file.clone();
        },
        _ => {
            error!("invalid media type: {}", media_type);
        }
    }
}

fn validate_media_type(dir: &str) -> &str {
    if dir.ends_with(".mp4")  || dir.ends_with(".mkv") || dir.ends_with(".mov") {
        "video"
    } else {
        "images"
    }
}
