use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::fs::File;
use crate::cli::Cli;

/// A struct of the server-script configurations. Serde will parse the configuration file with some default fields.
#[derive(Deserialize, Serialize)]
pub struct Configuration {
    #[serde(default = "default_server")]
    pub server: String,

    #[serde(default = "bool::default")]
    pub backup: bool,

    #[serde(default = "bool::default")]
    pub restart: bool,

    #[serde(default = "bool::default")]
    pub no_update: bool,

    #[serde(default = "memory")]
    pub memory: i32,

    #[serde(default = "Vec::new")]
    pub plugins: Vec<String>,

    #[serde(default = "Vec::new")]
    pub jvm_args: Vec<String>,

    #[serde(default = "HashMap::new")]
    pub servers: HashMap<String, String>
}

impl Configuration {
    pub fn apply(&mut self, cli: &Cli) {
        self.server = cli.server.clone();
        if cli.backup {
            self.backup = cli.backup;
        }
        self.memory = cli.memory;
        if cli.no_update {
            self.no_update = cli.no_update;
        }
    }
}

pub fn default_version() -> String {
    String::from("1.18")
}

/// The default server url
pub fn default_server() -> String {
    format!("https://paper-fetcher.netlify.app/paper?project=waterfall&version={}", default_version())
}

/// The default memory in Megabytes
pub fn memory() -> i32 {
    512
}

/// Loads the `server.conf.json` file and deserializes it to the `Configuration` struct.
pub async fn load_config() -> Result<Configuration, std::io::Error> {
    let path = Path::new("server.conf.json");

    // Create file if doesn't exists. Defaults to an empty object
    if !path.exists() {
        let _ = File::create(&path).await?;
        fs::write(&path, "{}").await?;
    }

    // Parse the configurations
    let data = serde_json::from_str::<Configuration>(
        fs::read_to_string(&path).await?
            .as_str()
    )?;

    // Pretty Print
    let data_str = serde_json::to_string_pretty::<Configuration>(&data)?;
    fs::write(path, data_str).await?;

    Ok(data)
}