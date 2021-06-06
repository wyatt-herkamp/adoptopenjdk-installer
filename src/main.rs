use crate::adoptopenjdk::AdoptOpenJDK;
use crate::adoptopenjdk::request::LatestBinary;
use crate::adoptopenjdk::response::{Architecture, HeapSize, Imagetype, JVMImpl, OS, ReleaseType, Vendor};
use std::path::Path;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use std::io::{stdin, stdout, Write};

pub mod utils;

pub mod adoptopenjdk;
pub mod installer;

#[tokio::main]
async fn main() {
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
    let result1 = jdk.download_binary(binary, std::env::temp_dir().as_path().clone()).await;
    if let Err(ref e) = result1 {
        println!("{}", e);
    }
    let buf = result1.unwrap();
    let result3 = installer::install(buf);
    if let Err(ref e) = result3 {
        println!("{}", e);
    }
}
