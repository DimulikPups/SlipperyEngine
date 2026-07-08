use std::fs;
use std::process::Command;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use log::{debug, error, info};
use crate::fs::{get_filename};

#[allow(dead_code)]
pub fn build_video_command(
    dir: &str,
    pattern: &str,
    fps: &str,
    codec: &str,
    pixformat: &str,
    output_name: &str,
) -> Command {
    debug!("building ffmpeg command.");
    let mut command = Command::new("ffmpeg");
    command
        .current_dir(dir)
        .arg("-r")
        .arg(fps)
        .arg("-i")
        .arg(pattern)
        .arg("-c:v")
        .arg(codec)
        .arg("-pix_fmt")
        .arg(pixformat)
        .arg("-y")
        .arg(output_name);
    command
}

pub fn extract_video_to_images_command(path: &str) -> Result<String, Error> {
    info!("{}",format!("extracting frames from {0}", get_filename(path).as_str()));

    let input_path = Path::new(path);
    let video_filename = input_path.file_name().unwrap().to_string_lossy().into_owned();
    let parent_dir = input_path.parent().unwrap().to_string_lossy().into_owned();

    let processing_name = format!("{0}_processing", video_filename);
    let frames_subdir = "frames";

    let processing_dir = Path::new(&parent_dir).join(&processing_name);
    let frames_dir = processing_dir.join(frames_subdir);

    debug!("parent_dir: {}", parent_dir);
    debug!("processing_dir: {:?}", processing_dir);
    debug!("frames_dir: {:?}", frames_dir);

    // Create directories
    fs::create_dir_all(&processing_dir).map_err(|e| {
        error!("failed to create processing directory: {}", e);
        Error::new(ErrorKind::Other, format!("failed to create processing directory: {}", e))
    })?;
    fs::create_dir_all(&frames_dir).map_err(|e| {
        error!("failed to create frames directory: {}", e);
        Error::new(ErrorKind::Other, format!("failed to create frames directory: {}", e))
    })?;

    // Verify directory was created
    if !frames_dir.exists() {
        error!("frames directory does not exist after creation: {:?}", frames_dir);
        return Err(Error::new(ErrorKind::Other, "frames directory was not created"));
    }

    // Use RELATIVE path from current_dir (parent_dir) with forward slashes
    // This avoids Windows path separator issues entirely
    let frames_pattern = format!("{0}/{1}/frame_%04d.png", processing_name, frames_subdir);
    debug!("ffmpeg output pattern (relative): {}", frames_pattern);

    let status = Command::new("ffmpeg")
        .current_dir(Path::new(&parent_dir))
        .arg("-i")
        .arg(&video_filename)
        .arg(&frames_pattern)
        .status()?;

    if !status.success() {
        error!("ffmpeg extracting frames failed");
        return Err(Error::new(ErrorKind::Other, "cant extract video to images"));
    }

    info!("frames extracted ({0})", path);
    Ok(frames_dir.to_string_lossy().into_owned())
}

pub fn images_to_video_command(
    frames_dir: &str,
    output_path: &str,
    fps: u32,
    codec: &str,
    extra_args: &[String],
) -> Result<(), Error> {
    info!("merging images from ({0}) to the video with {1} fps using codec {2}", frames_dir, fps, codec);

    let frames_dir_path = Path::new(frames_dir);
    let processing_dir = frames_dir_path.parent().unwrap();

    if !frames_dir_path.exists() {
        error!("frames directory does not exist: {}", frames_dir);
        return Err(Error::new(ErrorKind::NotFound, format!("frames directory not found: {}", frames_dir)));
    }

    let output_abs = if Path::new(output_path).is_absolute() {
        output_path.to_string()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| processing_dir.to_path_buf())
            .join(output_path)
            .to_string_lossy()
            .to_string()
    };

    if let Some(output_parent) = Path::new(&output_abs).parent() {
        if !output_parent.exists() {
            fs::create_dir_all(output_parent).map_err(|e| {
                error!("failed to create output directory: {}", e);
                Error::new(ErrorKind::Other, format!("failed to create output directory: {}", e))
            })?;
        }
    }

    let mut command = Command::new("ffmpeg");
    command
        .current_dir(processing_dir)
        .arg("-r")
        .arg(fps.to_string())
        .arg("-i")
        .arg("frames/frame_%04d.png")
        .arg("-c:v")
        .arg(codec);

    for arg in extra_args {
        command.arg(arg);
    }

    command
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-y")
        .arg(&output_abs);

    debug!("ffmpeg images_to_video command: {:?}", command);

    let output = command.output()?;
    if output.status.success() {
        info!("images succesfully merged to the {0}", output_abs);
        if processing_dir.exists() {
            fs::remove_dir_all(processing_dir)?;
        }
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        error!("ffmpeg merging failed. stderr: {}, stdout: {}", stderr, stdout);
        Err(Error::new(ErrorKind::Other, format!("ffmpeg merging failed: {}", stderr)))
    }
}

pub fn get_video_fps(path: &str) -> io::Result<f64> {
    debug!("getting video fps from: {}", path);
    let mut command = Command::new("ffprobe");
    command
        .arg("-v").arg("error")
        .arg("-select_streams").arg("v:0")
        .arg("-show_entries").arg("stream=avg_frame_rate")
        .arg("-of").arg("default=noprint_wrappers=1:nokey=1")
        .arg(path);

    let output = command.output().expect("failed to execute ffprobe");

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::new(ErrorKind::Other, err_msg));
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let trimmed = raw.trim();

    // Handle fractional format (eg, "30000/1001")
    if let Some((num_str, den_str)) = trimmed.split_once('/') {
        let num: f64 = num_str.parse()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Cannot parse numerator"))?;
        let den: f64 = den_str.parse()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Cannot parse denominator"))?;
        if den != 0.0 {
            return Ok(num / den);
        }
    }

    // Handle integer format
    let fps: f64 = trimmed.parse()
        .map_err(|_| Error::new(ErrorKind::InvalidData, "cant parse fps"))?;
    Ok(fps)
}
