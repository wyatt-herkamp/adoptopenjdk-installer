use serde::Deserialize;
use serde::Serialize;
use derive_more::Display;
use std::str::FromStr;
use crate::adoptopenjdk::AdoptOpenJDKError;
use std::fmt::{Display, Formatter};

use colored::*;

#[derive(Serialize, Deserialize, Debug, Clone,Display)]
pub enum Architecture {
    x64,
    x86,
    x32,
    ppc64,
    ppc64le,
    s390x,
    aarch64,
    arm,
    sparcv9,
    riscv64,
}

impl FromStr for Architecture {
    type Err = AdoptOpenJDKError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "x86" => Ok(Architecture::x86),
            "x32" => Ok(Architecture::x32),
            "x86_64" => Ok(Architecture::x64),
            _ => Err(AdoptOpenJDKError::Custom("Unable to find Architecture ".to_string()))
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone,Display)]
pub enum JVMImpl {
    hotspot,
    openj9,
}

#[derive(Serialize, Deserialize, Debug, Clone,Display)]
pub enum HeapSize {
    normal,
    large,
}

#[derive(Serialize, Deserialize, Debug, Clone,Display)]
pub enum Project {
    jdk,
    valhalla,
    metropolis,
    jfr,
    shenandoah,
}

#[derive(Serialize, Deserialize, Debug, Clone,Display)]
pub enum Imagetype {
    jdk,
    jre,
    testimage,
    debugimage,
    staticlibs,
}

#[derive(Serialize, Deserialize, Debug, Clone,Display)]
pub enum OS {
    linux,
    windows,
    mac,
    solaris,
    aix,
    #[serde(rename = "alpine-linux")]
    alpine_linux,
}

#[derive(Serialize, Deserialize, Debug, Clone,Display)]
pub enum ReleaseType {
    ea,
    ga,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvailableReleases {
    pub available_lts_releases: Vec<i64>,
    pub available_releases: Vec<i64>,
    pub most_recent_feature_release: i64,
    pub most_recent_feature_version: i64,
    pub most_recent_lts: i64,
    pub tip_version: i64,
}

impl Display for AvailableReleases {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut value = String::new();
        for x in self.available_releases.clone() {
            if self.available_lts_releases.contains(&x) {
                value = format!("{},{}", value, x.to_string().as_str().green());
            } else {
                value = format!("{},{}", value, x);
            }
        }
        value.remove(0);
        write!(f, "[{}]", value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone,Display)]
pub enum Vendor {
    adoptopenjdk,
    openjdk,
}