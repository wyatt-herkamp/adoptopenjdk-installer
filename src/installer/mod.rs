use std::path::{PathBuf, Path};
use crate::adoptopenjdk::AdoptOpenJDKError;
use std::fs::{File, create_dir_all};
use flate2::read::GzDecoder;
use tar::{Archive, Entry};
use std::process::Command;
use std::fs;

pub fn unpack(mut archive: Archive<GzDecoder<File>>, dst: &Path) -> Result<String, AdoptOpenJDKError> {


    // Canonicalizing the dst directory will prepend the path with '\\?\'
    // on windows which will allow windows APIs to treat the path as an
    // extended-length path with a 32,767 character limit. Otherwise all
    // unpacked paths over 260 characters will fail on creation with a
    // NotFound exception.
    let dst = &dst.canonicalize().unwrap_or(dst.to_path_buf());

    // Delay any directory entries until the end (they will be created if needed by
    // descendants), to ensure that directory permissions do not interfer with descendant
    // extraction.
    let mut first = None;
    let mut directories = Vec::new();
    for entry in archive.entries()? {
        let mut file = entry?;
        if file.header().entry_type() == tar::EntryType::Directory {
            if first.is_none() {
                first = Some(file.path().unwrap().to_str().unwrap().to_string());
            }
            directories.push(file);

        } else {
            file.unpack_in(dst)?;
        }
    }
    for mut dir in directories {
        dir.unpack_in(dst)?;
    }

    Ok(first.unwrap())
}

pub fn install(path: PathBuf) -> Result<bool, AdoptOpenJDKError> {
    println!("HEY");
    let file = File::open(path.clone())?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    let buf = Path::new("/opt").join("adoptopenjdk");
    if !buf.exists() {
        create_dir_all(&buf);
    }
    let string = unpack(archive, &buf)?;
    let mut path_two = buf.join(string);
    let java = path_two.clone().join("bin").join("java");
    let javac = path_two.clone().join("bin").join("javac");
    // sudo update-alternatives --install /usr/bin/java java <path> 1
    Command::new("update-alternatives").arg("--install").arg("/usr/bin/java").arg("java").arg(java.to_str().unwrap()).arg("1").spawn()?;
    Command::new("update-alternatives").arg("--install").arg("/usr/bin/javac").arg("javac").arg(javac.to_str().unwrap()).arg("1").spawn()?;
    Ok(true)
}