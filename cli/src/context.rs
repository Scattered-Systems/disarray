/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::Settings;
use serde::{Deserialize, Serialize};
use std::{
    convert::From,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub cnf: Settings,
    pub workdir: PathBuf,
}

impl Context {
    pub fn new(workdir: Option<PathBuf>) -> Self {
        let cnf = Settings::default();
        let workdir = workdir.unwrap_or_else(project_root);
        Self { cnf, workdir }
    }
    pub fn settings(&self) -> &Settings {
        &self.cnf
    }
    pub fn set_settings(&mut self, cnf: Settings) -> &Self {
        self.cnf = cnf;
        self
    }
    pub fn workdir(&self) -> &PathBuf {
        &self.workdir
    }
}

impl From<Settings> for Context {
    fn from(data: Settings) -> Self {
        Self {
            cnf: data,
            workdir: project_root(),
        }
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
