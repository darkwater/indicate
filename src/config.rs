extern crate config as rsconfig;

use color::Color;
use self::rsconfig::Value;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub struct Config {
    pub colors:       HashMap<String, Color>,
    pub status_items: Vec<String>,
    pub mpd:          MpdConfig,
    pub launch:       LaunchConfig,
}

pub struct MpdConfig {
    pub host: String,
    pub port: u16,
}

pub struct LaunchConfig {
    pub left:   Option<String>,
    pub middle: Option<String>,
    pub right:  Option<String>,
}

impl Config {
    pub fn default() -> Self {
        let mut config = rsconfig::Config::new();
        let mut config_path = PathBuf::from(env::var("HOME").unwrap_or(".".to_string()));
        config_path.push(".config/obsidian/config.toml");

        let _ = config.merge(rsconfig::File::from_str(include_str!("default_config.toml"), rsconfig::FileFormat::Toml)
                             .required(true));

        let _ = config.merge(rsconfig::File::new(config_path.to_str().unwrap(), rsconfig::FileFormat::Toml)
                             .required(false));

        Self::parse_rsconfig(config)
    }

    pub fn parse_rsconfig(config: rsconfig::Config) -> Self {
        let colors = config.get_table("colors").unwrap().into_iter().map(|(name, color)| {
            let errmsg = format!("invalid color {:?}", color);

            let color = color.into_str().unwrap();
            (name, color.parse().expect(&errmsg))
        }).collect();

        let status_items = config.get_array("status_items").unwrap().into_iter()
            .map(Value::into_str).map(Result::unwrap).collect();

        let mut mpd    = config.get_table("mpd").unwrap();
        let mut launch = config.get_table("launch").unwrap();

        Self {
            colors: colors,
            status_items: status_items,
            mpd: MpdConfig {
                host: mpd.remove("host").unwrap().try_into().unwrap(),
                port: mpd.remove("port").unwrap().try_into().unwrap(),
            },
            launch: LaunchConfig {
                left:   launch.remove("left").map(|c| c.try_into().unwrap()),
                middle: launch.remove("middle").map(|c| c.try_into().unwrap()),
                right:  launch.remove("right").map(|c| c.try_into().unwrap()),
            },
        }
    }

    pub fn get_color(&self, name: &str) -> Color {
        self.colors.get(name).expect(&format!("missing color: {}", name)).clone()
    }
}
