use crate::adoptopenjdk::AdoptOpenJDK;
use crate::adoptopenjdk::request::LatestBinary;
use crate::adoptopenjdk::response::{Architecture, HeapSize, Imagetype, JVMImpl, OS, ReleaseType, Vendor};
use std::path::Path;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use std::io::{stdin, stdout, Write};
use crate::installer::settings::Settings;
use clap::{App, Arg};
use crate::installer::Installer;

pub mod utils;

pub mod adoptopenjdk;
pub mod installer;

#[tokio::main]
async fn main() {
    if !whoami::username().eq("root") {
        println!("This applications must be ran as root!");
        return;
    }
    let installer = installer::Installer;
    if !installer.does_settings_exist() {
        let settings1 = Settings {
            install_location: "/opt/adoptopenjdk".to_string(),
            installs: vec![],
        };
        installer.update_settings(settings1).unwrap();
    }
    let mut app = App::new("AdoptOpenJDK Installer").
        version("0.1.0").author("Wyatt Jacob Herkamp <wherkamp@kingtux.me>").about("A AdoptOpenJDK installer for Linux")
        .arg(Arg::with_name("install").short("i").long("install").help("Install a Java Version").takes_value(false))
        .arg(Arg::with_name("list").short("l").long("list").help("Lists installed Java versions").takes_value(false))
        .arg(Arg::with_name("remove").short("r").long("remove").help("Remove A Java Install").takes_value(false));
    let mut matches = app.clone().get_matches();
    if matches.is_present("install") {
        install(&installer).await;
    } else if matches.is_present("list") {
        let settings = installer.get_settings().unwrap();
        for x in settings.installs {
            println!("{}", x);
        }
    } else if matches.is_present("remove") {} else {
        app.print_help();
    }
}

pub async fn install(installer: &Installer) {
    let jdk = AdoptOpenJDK::new("AdoptOpenJDK Installer by Wyatt Herkamp (github.com/wherkamp)".to_string());
    let result = jdk.get_releases().await.unwrap();
    print!("Please Select a Java Version {}: ", result.to_string());
    let mut java_version = String::new();
    stdout().flush();
    let result = stdin().read_line(&mut java_version);
    if let Err(err) = result {
        panic!("Fail {}", err);
    }
    java_version.truncate(java_version.len() - 1);
    let arch = std::env::consts::ARCH;
    let value = i64::from_str(java_version.as_str());
    if let Err(ref err) = value {
        println!("{}", err);
    }
    let binary = LatestBinary {
        arch: Architecture::from_str(arch).unwrap(),
        feature_version: value.unwrap(),
        heap_size: HeapSize::normal,
        image_type: Imagetype::jdk,
        jvm_impl: JVMImpl::hotspot,
        os: OS::linux,
        release_type: ReleaseType::ga,
        vendor: Vendor::adoptopenjdk,
    };
    let install = installer::settings::Install {
        jvm_version: binary.feature_version.clone(),
        jvm_impl: binary.jvm_impl.clone(),
        location: "".to_string(),
    };
    let result2 = installer.contains_install(&install).unwrap();
    if result2 {
        println!("That version has already been installed");
        return;
    }
    let result1 = jdk.download_binary(binary.clone(), std::env::temp_dir().as_path().clone()).await;
    if let Err(ref e) = result1 {
        println!("{}", e);
    }
    let buf = result1.unwrap();
    let installer = installer::Installer;
    let result3 = installer.install(buf, install);
    if let Err(ref e) = result3 {
        println!("{}", e);
    }
}
