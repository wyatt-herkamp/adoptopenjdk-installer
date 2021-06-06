use serde::Deserialize;
use serde::Serialize;
use crate::adoptopenjdk::response::JVMImpl;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub install_location: String,
    pub installs: Vec<Install>,
}
impl Settings{
    pub fn add_install(&mut self, install: Install){
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