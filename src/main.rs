pub mod fs;
pub mod ffmpeg;
pub mod engine;
pub mod pipeline;
pub mod python;

#[cfg(test)]
pub mod tests;

use std::env;
use log::{error, info};
use crate::pipeline::{Builder};

#[derive(Debug)]
pub struct ParsedArgs {
    pub input_file: String,
    pub vfitype: String,
    pub typeofexecution: i8,
    pub fps: f32,
    pub scale: f32,
    pub multi: i8,
    pub fp16: bool,
    pub union: bool,
}

impl<'a> IntoIterator for &'a ParsedArgs {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let args = vec![
            "-i".to_string(), self.input_file.clone(),
            "-vfitype".to_string(), self.vfitype.clone(),
            "-type".to_string(), self.typeofexecution.to_string(),
            "-fps".to_string(), self.fps.to_string(),
            "-scale".to_string(), self.scale.to_string(),
            "-multi".to_string(), self.multi.to_string(),
            "-fp16".to_string(), self.fp16.to_string(),
            "-union".to_string(), self.union.to_string(),
        ];

        args.into_iter()
    }
}

impl Default for ParsedArgs {
    fn default() -> Self {
        Self {
            input_file: String::new(),
            vfitype: String::new(),
            typeofexecution: 0,
            fps: 0.0,
            scale: 1.0,
            multi: 2,
            fp16: false,
            union: false,
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
            "-vfitype" => parsed_args.vfitype = value.clone(),
            "-type" => parsed_args.typeofexecution = value.parse::<i8>().map_err(|e| e.to_string())?,
            "-fps" => parsed_args.fps = value.parse::<f32>().map_err(|_| "Failed to parse fps argument".to_string())?,
            "-scale" => parsed_args.scale = value.parse::<f32>().map_err(|_| "Failed to parse scale argument".to_string())?,
            "-multi" => parsed_args.multi = value.parse::<i8>().map_err(|_| "Failed to parse multi argument".to_string())?,
            "-fp16" => parsed_args.fp16 = value.parse::<bool>().map_err(|_| "Failed to parse fp16 argument".to_string())?,
            "-union" => parsed_args.union = value.parse::<bool>().map_err(|_| "Failed to parse union argument".to_string())?,
            _ => return Err(format!("Invalid argument: {}", args[i])),
        }

        i += 2;
    }

    if parsed_args.input_file.is_empty() {
        return Err("No input file provided.".to_string());
    }
    if parsed_args.vfitype.is_empty() {
        return Err("No VFI model type provided.".to_string());
    }
    if parsed_args.fps <= 0.0 {
        return Err("No valid fps provided.".to_string());
    }

    Ok(parsed_args)
}

fn main() {
    pretty_env_logger::init();
    info!("Initialized. Collecting arguments.");
    let args: Vec<String> = env::args().collect();
    let parsed_args = parse_args(&args);

    match parsed_args {
        Ok(parsed_args) => {
            let (pipeline, context) = Builder::new()
                .with_interpolation()
                .with_scale(parsed_args.scale)
                .with_union(parsed_args.union)
                .with_fp16(parsed_args.fp16)
                .build();

            let context = context
                .with_original_fps(parsed_args.fps)
                .with_multiply(parsed_args.multi);

            engine::execute(&pipeline, context);
        },
        Err(err) => error!("{}", err),
    }
}
