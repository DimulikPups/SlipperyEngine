//! Python runtime setup and management.

use std::process::{Command, Stdio};
use std::error::Error;
use std::fs;
use tokio::io;
use tokio::fs as tokio_fs;
use log::info;

use crate::fs::unzip;
use crate::ParsedArgs;

const PYTHON_VERSION: &str = "3.12.2";
const PYTHON_DIR: &str = "src/python";
const PYTHON_PACKED_DIR: &str = "src/python/python-packed";
const PYTHON_EXE: &str = "src/python/python-packed/python.exe";
const PYTHON_PTH: &str = "src/python/python-packed/python312._pth";
const GET_PIP_SCRIPT: &str = "src/python/get-pip.py";
const REQUIREMENTS_FILE: &str = "src/python/requirements.txt";

pub async fn run_script(script_dir: &str, parsed_args: ParsedArgs) -> Result<(), Box<dyn Error>> {
    let mut process = tokio::process::Command::new(PYTHON_EXE)
        .arg(script_dir)
        .args(&parsed_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = process.wait().await?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Cannot execute script: {:?}", script_dir).into())
    }
}

pub async fn ensure_python_ready() -> Result<(), Box<dyn Error>> {
    if !fs::exists(PYTHON_PACKED_DIR)? {
        info!("python not found. downloading...");
        tokio_fs::create_dir_all(PYTHON_DIR).await?;
        write_requirements().await?;
        download_python(PYTHON_DIR).await?;
        info!("python & pip downloaded. extracting to {}", PYTHON_PACKED_DIR);
        unzip("src/python/python-packed.zip", PYTHON_PACKED_DIR)?;
        enable_site_packages()?;
        info!("python extracted. verifying...");
        check_python_version()?;
        install_pip()?;
        info!("installing python dependencies...");
        download_dependencies().await?;
        info!("python dependencies installed. ready!");
        Ok(())
    } else {
        write_requirements().await?;
        enable_site_packages()?;
        info!("python runtime found at {}", PYTHON_PACKED_DIR);
        Ok(())
    }
}

async fn write_requirements() -> Result<(), Box<dyn Error>> {
    let requirements = "\
cupy-cuda11x
matplotlib
moviepy
numpy>=1.26,<2
opencv-python
rawpy
scipy
scikit-video
scikit-image
torch
torchvision
tqdm
tensorboard
lpips";

    tokio_fs::write(REQUIREMENTS_FILE, requirements).await?;
    Ok(())
}

async fn download_python(dir: &str) -> Result<(), Box<dyn Error>> {
    let version = PYTHON_VERSION;
    let url = format!("https://www.python.org/ftp/python/{}/python-{}-embed-amd64.zip", version, version);

    let bytes = reqwest::get(url).await?.bytes().await?;
    tokio_fs::write(format!("{}/python-packed.zip", dir), bytes).await?;

    let bytes = reqwest::get("https://bootstrap.pypa.io/get-pip.py").await?.bytes().await?;
    tokio_fs::write(format!("{}/get-pip.py", dir), bytes).await?;

    Ok(())
}

pub async fn download_dependencies() -> Result<(), Box<dyn Error>> {
    let mut cmd = tokio::process::Command::new(PYTHON_EXE)
        .args(["-m", "pip", "install", "--no-warn-script-location", "-r", REQUIREMENTS_FILE])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = cmd.wait().await?;

    if !status.success() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "failed to install dependencies",
        )));
    }
    Ok(())
}

pub fn install_pip() -> Result<(), Box<dyn Error>> {
    let status = Command::new(PYTHON_EXE)
        .arg(GET_PIP_SCRIPT)
        .arg("--no-warn-script-location")
        .status()?;

    if !status.success() {
        return Err(format!("get-pip.py exited with {}", status).into());
    }
    Ok(())
}

fn enable_site_packages() -> Result<(), Box<dyn Error>> {
    let pth = fs::read_to_string(PYTHON_PTH)?;

    if pth.contains("#import site") {
        let updated = pth.replace("#import site", "import site");
        fs::write(PYTHON_PTH, updated)?;
    }

    Ok(())
}

pub fn check_python_version() -> Result<(), std::io::Error> {
    let status = Command::new(PYTHON_EXE)
        .arg("--version")
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other("python check failed"))
    }
}
