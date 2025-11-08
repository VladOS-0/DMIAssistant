use std::{
    env,
    fs::{OpenOptions, create_dir_all},
    io::{Read, Write},
    path::PathBuf,
};

use iced::Color;
use serde::{Deserialize, Serialize};

use crate::{
    dmi_utils::CustomFilterType,
    screens::{
        explorer::ExplorerSettings,
        viewer::{StateboxResizing, StateboxSettings},
    },
    utils::{Directories, get_project_dir},
};

const CONFIG_FILE_NAME: &str = "Config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableStateboxSettings {
    pub linear_background_color: [f32; 4],
    pub linear_text_color: [f32; 4],

    pub debug: bool,
    pub animated: bool,
    pub show_resized: bool,

    pub resize: StateboxResizing,
    pub filter_type: Option<CustomFilterType>,
}

impl From<StateboxSettings> for SerializableStateboxSettings {
    fn from(value: StateboxSettings) -> Self {
        Self {
            linear_background_color: value.background_color.into_linear(),
            linear_text_color: value.text_color.into_linear(),
            debug: value.debug,
            animated: value.animated,
            show_resized: value.show_resized,
            resize: value.resize,
            filter_type: value.filter_type,
        }
    }
}

impl From<SerializableStateboxSettings> for StateboxSettings {
    fn from(value: SerializableStateboxSettings) -> Self {
        Self {
            background_color: Color::from_linear_rgba(
                value.linear_background_color[0],
                value.linear_background_color[1],
                value.linear_background_color[2],
                value.linear_background_color[3],
            ),
            text_color: Color::from_linear_rgba(
                value.linear_text_color[0],
                value.linear_text_color[1],
                value.linear_text_color[2],
                value.linear_text_color[3],
            ),
            debug: value.debug,
            animated: value.animated,
            show_resized: value.show_resized,
            resize: value.resize,
            filter_type: value.filter_type,
        }
    }
}

impl Default for SerializableStateboxSettings {
    fn default() -> Self {
        Self::from(StateboxSettings::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub path_to_config_file: PathBuf,
    pub log_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub data_dir: PathBuf,
    // for viewer screen
    pub statebox_defaults: SerializableStateboxSettings,
    // for extractor screen
    pub explorer_settings: ExplorerSettings,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path_to_config_file: get_project_dir(Directories::Config)
                .join(CONFIG_FILE_NAME),
            log_dir: get_project_dir(Directories::Log),
            cache_dir: get_project_dir(Directories::Cache),
            data_dir: get_project_dir(Directories::Data),
            statebox_defaults: SerializableStateboxSettings::default(),
            explorer_settings: ExplorerSettings::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path_to_config =
            env::var("CONFIG_PATH").map(|path| path.into()).unwrap_or(
                get_project_dir(Directories::Config).join(CONFIG_FILE_NAME),
            );

        let mut buf: String = String::new();
        if let Ok(mut file) =
            OpenOptions::new().read(true).open(&path_to_config)
            && file.read_to_string(&mut buf).is_ok_and(|bytes| bytes > 0)
            && let Ok(loaded_config) = toml::from_str::<Config>(&buf)
        {
            return loaded_config;
        };

        let new_config = Self::default();
        new_config.save();
        new_config
    }

    pub fn save(&self) {
        create_dir_all(self.path_to_config_file.parent().unwrap()).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.path_to_config_file)
            .unwrap();
        file.write_all(toml::to_string_pretty(self).unwrap().as_bytes())
            .unwrap();
    }
}
