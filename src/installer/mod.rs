pub mod settings;

use std::path::{PathBuf, Path};
use crate::adoptopenjdk::AdoptOpenJDKError;
use std::fs::{File, create_dir_all, read_to_string, OpenOptions};
use flate2::read::GzDecoder;
use tar::{Archive, Entry};
use std::process::Command;
use std::fs;
use crate::installer::settings::{Install, Settings};
use std::io::Write;

/// Stolen from Tar depend and modified to fit my needs
pub fn unpack(mut archive: Archive<GzDecoder<File>>, dst: &Path) -> Result<String, AdoptOpenJDKError> {
    let dst = &dst.canonicalize().unwrap_or(dst.to_path_buf());
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

pub struct Installer;

impl Installer {
    pub fn install(&self, path: PathBuf, install: Install) -> Result<bool, AdoptOpenJDKError> {
        println!("Installing: {} {}", install.jvm_version, install.jvm_impl.to_string());
        let file = File::open(path.clone())?;
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        let mut settings = self.get_settings()?;

        let buf = Path::new(settings.install_location.as_str()).join(install.jvm_impl.to_string());
        if !buf.exists() {
            create_dir_all(&buf);
        }
        let string = unpack(archive, &buf)?;
        let mut path_two = buf.join(string);
        let mut install = install.clone();
        install.set_location(path_two.clone().to_str().unwrap().to_string());
        settings.add_install(install);
        self.update_settings(settings);
        let java = path_two.clone().join("bin").join("java");
        let javac = path_two.clone().join("bin").join("javac");
        let javadoc = path_two.clone().join("bin").join("javadoc");
        // sudo update-alternatives --install /usr/bin/java java <path> 1
        Command::new("chmod").arg("-Rv").arg("755").arg(path_two.to_str().unwrap()).spawn()?;
        Command::new("update-alternatives").arg("--install").arg("/usr/bin/java").arg("java").arg(java.to_str().unwrap()).arg("1").spawn()?;
        Command::new("update-alternatives").arg("--install").arg("/usr/bin/javac").arg("javac").arg(javac.to_str().unwrap()).arg("1").spawn()?;
        Command::new("update-alternatives").arg("--install").arg("/usr/bin/javadoc").arg("javadoc").arg(javadoc.to_str().unwrap()).arg("1").spawn()?;
        Ok(true)
    }
    pub fn get_settings(&self) -> Result<Settings, AdoptOpenJDKError> {
        let buf = Path::new("/etc").join("adoptopenjdk").join("settings.toml");
        let result = read_to_string(buf)?;
        return toml::from_str(result.as_str()).map_err(AdoptOpenJDKError::from);
    }
    pub fn update_settings(&self, settings: Settings) -> Result<(), AdoptOpenJDKError> {
        let buf = Path::new("/etc").join("adoptopenjdk").join("settings.toml");
        if !buf.exists() {
            let x = buf.parent().unwrap();
            if !x.exists() {
                create_dir_all(x)?;
            }
        }
        let string = toml::to_string(&settings)?;
        let mut file = OpenOptions::new().write(true).read(true).create(true).open(buf)?;
        file.write_all(string.as_bytes())?;
        Ok(())
    }
    pub fn does_settings_exist(&self) -> bool {
        Path::new("/etc").join("adoptopenjdk").join("settings.toml").exists()
    }
}
