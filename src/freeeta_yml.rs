use serde::{Deserialize, Serialize};
use std::fs::{self, File};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FreeEtaConfig {
    /// Software name: typically "FreeEta"
    name: String,
    /// Language: ["zh-cn", "en-us"]
    lang: String,
    /// Theme: Default "Light"; Others see [iced::theme::Theme](https://docs.rs/iced/0.13.1/iced/theme/enum.Theme.html)
    theme: String,
}

/// Read FreeEta config from ./
pub fn read_freeeta_config() -> Result<(), serde_yml::Error> {
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
    } else {
        // If config exists, read from it.
        let contents = fs::read_to_string(path).expect("Cannot read config.yml");
        let deserialized_config: FreeEtaConfig = serde_yml::from_str(&contents)?;
        // println!("{:?}", deserialized_config);
    }
    Ok(())
}
