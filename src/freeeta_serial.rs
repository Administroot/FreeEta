use serde::{Deserialize, Serialize};
use std::fs::{self, File};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Metadata of all json file
pub struct MetaData {
    name: String,
    api_version: String,
    company: String,
    staff: String,
    date: String,
    phone: String,
    mail: String,
}

#[allow(dead_code)]
impl MetaData {
    pub fn new() -> Self {
        MetaData {
            name: String::from("Eta-Name"),
            api_version: String::from("0.1.0"),
            company: String::from("Company-You-Work-For"),
            staff: String::from("Your-Name"),
            date: String::from("Assign-A-Date"),
            phone: String::from("Your-Phone"),
            mail: String::from("Your-Email"),
        }
    }
}

/// Nodes of ETA
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EtaVector {
    /// Node name (e.g. "Valve").
    name: String,
    /// Success probability of node
    rate: f32,
    /// Metadata of the node in user interface
    pic: PictureNode,
    /// Previous node. `All` nodes are connected as a tree
    prev: Vec<String>,
}

#[allow(dead_code)]
impl EtaVector {
    pub fn new() -> Self {
        EtaVector {
            name: String::new(),
            rate: 0f32,
            pic: PictureNode::new(),
            prev: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Movable node contains a picture
struct PictureNode {
    /// Relative path of the picture
    picture: String,
    /// The postion of the node
    axis: Axis,
}

#[allow(dead_code)]
impl PictureNode {
    pub fn new() -> Self {
        PictureNode {
            picture: String::new(),
            axis: Axis::new(0., 0.),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Axis<T = f32> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Axis<T> {
    pub fn new(x: T, y: T) -> Self {
        Axis { x, y }
    }
}

// impl Serialize for iced::Point {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//         let mut point = serializer.serialize_struct("axis", 2)?;
//         point.serialize_field("x", &self.x)?;
//         point.serialize_field("y", &self.y)?;
//         point.end()
//     }
// }

// impl Deserialize for iced::Point {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//         deserializer.deserialize_struct("axis", ["x", "y"], visitor)
//     }
// }

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

#[allow(dead_code)]
impl FreeEtaConfig {
    pub fn new() -> Self {
        FreeEtaConfig {
            name: String::from("FreeEta"),
            lang: String::from("zh-cn"),
            theme: String::from("Light"),
        }
    }
}

#[allow(dead_code)]
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
    pub meta: MetaData,
    pub nodes: Vec<EtaVector>,
}

#[allow(dead_code)]
impl EtaEntity {
    pub fn new() -> Self {
        EtaEntity {
            meta: MetaData::new(),
            nodes: Vec::new(),
        }
    }
}

/// Deserialize Eta file.
pub fn read_eta_json(path: &str) -> Result<EtaEntity, serde_json::Error> {
    let content = fs::read_to_string(path).expect("Json syntax error");
    let eta_json: EtaEntity = serde_json::from_str(&content)?;
    Ok(eta_json)
}
