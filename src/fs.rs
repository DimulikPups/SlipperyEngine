use std::fs;
use std::io::{self};
use std::path::Path;
use zip::ZipArchive;

pub fn create_dir(path: &str, name: &str) -> io::Result<String> {
    let full_path = Path::new(path).join(name);
    fs::create_dir_all(&full_path)?;
    Ok(full_path.to_string_lossy().into_owned())
}

pub fn unzip(archive_path: &str, dest: &str) -> zip::result::ZipResult<()> {
    let file = fs::File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(dest).join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

pub fn get_parent_path(dir: &str) -> String {
    let parent_path = Path::new(dir).parent().unwrap().to_string_lossy().into_owned();
    parent_path
}

pub fn get_filename(dir: &str) -> String {
    let filename = Path::new(dir).file_name().unwrap().to_string_lossy().into_owned();
    filename
}
