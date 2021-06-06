use serde::Deserialize;
use serde::Serialize;
use crate::adoptopenjdk::response::{Architecture, HeapSize, Imagetype, JVMImpl, OS, Project, ReleaseType, Vendor};

pub struct LatestBinary {
    pub arch: Architecture,
    pub feature_version: i64,
    pub heap_size: HeapSize,
    pub image_type: Imagetype,
    pub jvm_impl: JVMImpl,
    pub os: OS,
    pub release_type: ReleaseType,
    pub vendor: Vendor,

}

impl ToString for LatestBinary {
    fn to_string(&self) -> String {
        format!("{feature_version}/{release_type}/{os}/{arch}/{image_type}/{jvm_impl}/{heap_size}/{vendor}",
                arch = self.arch,
                feature_version = self.feature_version,
                release_type = self.release_type,
                os = self.os,
                image_type = self.image_type,
                jvm_impl = self.jvm_impl,
                heap_size = self.heap_size,
                vendor = self.vendor)
    }
}