use serde::{Deserialize, Serialize};
use std::fs::{self, File};

/// The config struct of FreeEta
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FreeEtaConfig {
    /// Software name: typically "FreeEta"
    name: String,
    /// Language: ["zh-cn", "en-us"]
    lang: String,
    /// Theme: Default "Light"; Others see [iced::theme::Theme](https://docs.rs/iced/0.13.1/iced/theme/enum.Theme.html)
    theme: String,
}

/// Read FreeEta config from ./
pub fn read_freeeta_config() -> Result<FreeEtaConfig, serde_yml::Error> {
    let path = "config.yml";
    if !fs::exists(path).unwrap() {
        // If config doesn't exist, create a default config.
        File::create(path).unwrap();
        let default_config = FreeEtaConfig {
            name: "FreeEta".to_string(),
            lang: "zh-cn".to_string(),
            theme: "Light".to_string(),
        };
        let yaml = serde_yml::to_string(&default_config)?;
        fs::write(path, yaml).expect("Cannot write config.yml");
        return Ok(default_config);
    } else {
        // If config exists, read from it.
        let content = fs::read_to_string(path).expect("YAML syntax error");
        let deserialized_config: FreeEtaConfig = serde_yml::from_str(&content)?;
        return Ok(deserialized_config);
    }
}

/// ETA user personalized
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EtaEntity {
    /// Eta name
    name: String,
    api_version: String,
    company: String,
    staff: String,
    date: String,
    phone: String,
    mail: String,
    pub nodes: Vec<EtaNode>,
}

impl EtaEntity {
    pub fn new() -> Self {
        EtaEntity {
            name: String::from("Eta-Name"),
            api_version: String::from("0.1.0"),
            company: String::from("Company-You-Work-For"),
            staff: String::from("Your-Name"),
            date: String::from("Assign-A-Date"),
            phone: String::from("Your-Phone"),
            mail: String::from("Your-Email"),
            nodes: Vec::new(),
        }
    }
}

/// Nodes of ETA
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EtaNode {
    /// Node name (e.g. "Valve").
    name: String,
    /// Success probability of node
    rate: f32,
    /// Previous node. `All` nodes are connected as a tree
    prev: Vec<String>,
}

/// Deserialize Eta file.
pub fn read_eta_json(path: &str) -> Result<EtaEntity, serde_json::Error> {
    let content = fs::read_to_string(path).expect("Json syntax error");
    let eta_json: EtaEntity = serde_json::from_str(&content)?;
    Ok(eta_json)
}
