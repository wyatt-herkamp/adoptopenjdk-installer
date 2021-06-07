use std::{fs,
          path::Path,
};
use std::fs::create_dir_all;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, header};
use reqwest::Url;
use crate::adoptopenjdk::AdoptOpenJDKError;
use std::path::PathBuf;

// Stolen from Kakara's Klauncher https://github.com/kakaragame/KLauncher/blob/master/src/downloader.rs
// By Wyatt Herkamp and Ryandw11
pub async fn download(url: &str, location: &Path, what: &str) -> Result<PathBuf, AdoptOpenJDKError> {
    println!("Downloading {}", what);
    let x = location.clone();
    if !x.exists() {
        create_dir_all(x)?;
    }
    let url = Url::parse(url).unwrap();
    let client = Client::new();
    let total_size = {
        let resp = client.head(url.as_str()).send().await.unwrap();
        if resp.status().is_success() {
            let value: String;
            if let Some(e) = resp.headers().get(header::CONTENT_DISPOSITION) {
                let result = e.to_str().unwrap();
                let split = result.split("; ").collect::<Vec<&str>>();
                let option = split.get(1);
                let split1 = option.unwrap().split("=");
                let vec = split1.collect::<Vec<&str>>();
                let x1 = vec.get(1).unwrap();
                value = x1.to_string();
            } else {
                return Err(AdoptOpenJDKError::Custom("Fail".to_string()));
            }
            let x2 = resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0);
            (x2, value)
        } else {
            return Err(AdoptOpenJDKError::HTTPError(resp.status()));
        }
    };
    let location = location.join(total_size.1);
    let mut request = client.get(url.as_str());
    let pb = ProgressBar::new(total_size.0);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));


    if location.exists() {
        let size = &location.metadata().unwrap().len() - 1;
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }


    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&location).unwrap();


    let mut source = request.send().await.unwrap();
    while let Some(chunk) = source.chunk().await.unwrap() {
        dest.write_all(&chunk)?;
        pb.inc(chunk.len() as u64);
    }
    println!(
        "Download of '{}' has been completed.",
        location.clone().to_str().unwrap()
    );

    Ok(location)
}
