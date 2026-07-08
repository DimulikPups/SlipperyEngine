use std::process::Command;
use std::io::{self, Error, ErrorKind};
use log::{debug, info};

pub fn run_command(mut command: Command) -> io::Result<()> {
    info!("Running command: {:?}", &command);
    command.status()?;
    Ok(())
}

pub fn build_video_command(
    dir: &str,
    pattern: &str,
    fps: &str,
    codec: &str,
    pixformat: &str,
    output_name: &str,
) -> Command {
    debug!("Building ffmpeg command.");
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

pub fn get_video_fps(path: &str) -> io::Result<f64> {
    debug!("Getting video fps from: {}", path);
    let mut command = Command::new("ffprobe");
    command
        .arg("-v").arg("error")
        .arg("-select_streams").arg("v:0")
        .arg("-show_entries").arg("stream=avg_frame_rate")
        .arg("-of").arg("default=noprint_wrappers=1:nokey=1")
        .arg(path);

    let output = command.output().expect("Failed to execute ffprobe");

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::new(ErrorKind::Other, err_msg));
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let trimmed = raw.trim();

    // Handle fractional format (e.g., "30000/1001")
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
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Cannot parse fps"))?;
    Ok(fps)
}
