use serde::Deserialize;
use serde::Serialize;
use crate::adoptopenjdk::response::JVMImpl;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub install_location: String,
    pub installs: Vec<Install>,
}

impl Display for Settings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = serde_json::to_string(self).unwrap();
        write!(f, "{}", result)
    }
}

impl Settings {
    pub fn add_install(&mut self, install: Install) {
        self.installs.push(install);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Install {
    pub jvm_version: i64,
    pub jvm_impl: JVMImpl,
    pub location: String,
}

impl Install {
    pub fn set_location(&mut self, location: String) {
        self.location = location;
    }
}

impl Display for Install {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = serde_json::to_string(self).unwrap();
        write!(f, "{}", result)
    }
}

impl PartialEq for Install {
    fn eq(&self, other: &Install) -> bool {
        self.jvm_impl == other.jvm_impl && self.jvm_version == other.jvm_version
    }
}
