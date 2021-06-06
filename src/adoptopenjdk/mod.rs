use reqwest::{StatusCode, Response, Body, Client};
use std::error::Error;
use std::fmt::{Formatter, Display};
use crate::adoptopenjdk::response::AvailableReleases;
use serde::de::DeserializeOwned;
use reqwest::header::{USER_AGENT, HeaderValue, HeaderMap};
use std::fs::File;
use std::path::{Path, PathBuf};
use crate::utils::utils;

pub mod response;
pub mod request;

#[derive(Debug)]
pub enum AdoptOpenJDKError {
    HTTPError(StatusCode),
    ReqwestError(reqwest::Error),
    JSONError(serde_json::Error),
    STDIoError(std::io::Error),
    Custom(String),
}

impl Display for AdoptOpenJDKError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            AdoptOpenJDKError::HTTPError(status) => {
                let string = format!("The API returned a non-success error code {}", status.clone().as_str());
                return write!(f, "{}", string);
            }
            AdoptOpenJDKError::ReqwestError(_) => {
                let x = "An error occurred while processing the HTTP response";
                return write!(f, "{}", x);
            }
            AdoptOpenJDKError::JSONError(_) => {
                let x = "The JSON sent by AdoptOpenJDK did not match what this app was expecting";
                return write!(f, "{}", x);
            }
            AdoptOpenJDKError::Custom(s) => {
                return write!(f, "{}", s.as_str());
            }
            AdoptOpenJDKError::STDIoError(err) => {
                let x = "IO Error";
                return write!(f, "{} {}", x, err);
            }
        }
    }
}

impl Error for AdoptOpenJDKError {}

impl From<reqwest::Error> for AdoptOpenJDKError {
    fn from(err: reqwest::Error) -> AdoptOpenJDKError {
        AdoptOpenJDKError::ReqwestError(err)
    }
}
impl From<std::io::Error> for AdoptOpenJDKError {
    fn from(err: std::io::Error) -> AdoptOpenJDKError {
        AdoptOpenJDKError::STDIoError(err)
    }
}

impl From<serde_json::Error> for AdoptOpenJDKError {
    fn from(err: serde_json::Error) -> AdoptOpenJDKError {
        AdoptOpenJDKError::JSONError(err)
    }
}

pub struct AdoptOpenJDK {
    client: Client,
    user_agent: String,
}

impl AdoptOpenJDK {
    pub fn new(
        user_agent: String,
    ) -> AdoptOpenJDK {
        let client = Client::new();
        AdoptOpenJDK {
            client,
            user_agent,
        }
    }
    pub async fn get(&self, url: &str) -> Result<Response, reqwest::Error> {
        let string = self.build_url(url);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&*self.user_agent).unwrap(),
        );
        self.client.get(string).headers(headers).send().await
    }
    /// Makes a post request with Reqwest response
    pub async fn post(
        &self,
        url: &str,
        body: Body,
    ) -> Result<Response, reqwest::Error> {
        let string = self.build_url(url);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&*self.user_agent).unwrap(),
        );
        self.client
            .post(string)
            .body(body)
            .headers(headers)
            .send()
            .await
    }
    /// Makes a get request with JSON response
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        url: &str) -> Result<T, AdoptOpenJDKError> {
        let x = self.get(url).await;
        return AdoptOpenJDK::respond::<T>(x).await;
    }
    /// Makes a post request with JSON response
    pub async fn post_json<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Body,
    ) -> Result<T, AdoptOpenJDKError> {
        let x = self.post(url, body).await;
        return AdoptOpenJDK::respond::<T>(x).await;
    }
    /// Builds a URL
    pub fn build_url(&self, dest: &str) -> String {
        format!("https://api.adoptopenjdk.net/v3/{}", dest)
    }
    /// Handles a Response from Reqwest mainly for internal use
    pub async fn respond<T: DeserializeOwned>(
        result: Result<Response, reqwest::Error>,
    ) -> Result<T, AdoptOpenJDKError> {
        if let Ok(response) = result {
            let code = response.status();
            if !code.is_success() {
                return Err(AdoptOpenJDKError::HTTPError(code));
            }

            let value = response.json::<T>().await;
            if let Ok(about) = value {
                return Ok(about);
            } else if let Err(response) = value {
                return Err(AdoptOpenJDKError::from(response));
            }
        } else if let Err(response) = result {
            return Err(AdoptOpenJDKError::from(response));
        }
        return Err(AdoptOpenJDKError::Custom("IDK".to_string()));
    }

    pub async fn get_releases(&self) -> Result<AvailableReleases, AdoptOpenJDKError> {
        return self.get_json("info/available_releases").await;
    }
    pub async fn download_binary(&self, request: request::LatestBinary, file: &Path) -> Result<PathBuf, AdoptOpenJDKError> {
        let x = self.build_url(format!("binary/latest/{}", request.to_string()).as_str());
        println!("{}", &x);
        let result = utils::download(x.as_str(), file, format!("Adopt Open JDK {}", request.to_string()).as_str()).await;
        return result;
    }
}