use std::fs;
use std::io::{self};
use std::path::Path;
use zip::ZipArchive;

pub fn create_dir(path: &str, name: &str) -> io::Result<String> {
    let full_path = format!("{}/{}", path, name);
    fs::create_dir_all(&full_path)?;
    Ok(full_path)
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
