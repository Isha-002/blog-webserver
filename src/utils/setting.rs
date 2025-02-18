use anyhow::Result;
use config::{Config, File, FileFormat};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, io::Write, path::Path, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Default)]
#[allow(non_camel_case_types)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    debug,
    #[default]
    info,
    warn,
    error,
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::debug),
            "info" => Ok(LogLevel::info),
            "warn" => Ok(LogLevel::warn),
            "error" => Ok(LogLevel::error),
            _ => Ok({
                println!("Invalid log level, Defaulting to INFO");
                LogLevel::info
            }),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let log_str = match self {
            LogLevel::debug => "debug",
            LogLevel::info => "info",
            LogLevel::warn => "warn",
            LogLevel::error => "error",
        };
        write!(f, "{}", log_str)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ServerConfig {
    #[serde(default = "default_db_url")]
    pub db_url: String,
    #[serde(default = "default_server_port")]
    pub server_port: String,
    #[serde(default = "default_origin_port")]
    pub origin_port: String,
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
}
fn default_db_url() -> String {
    "postgres://postgres:4431@localhost:5432/blogs".into()
}
fn default_server_port() -> String {
    "4445".into()
}

fn default_origin_port() -> String {
    "4446".into()
}

fn default_log_level() -> LogLevel {
    LogLevel::info
}

pub fn config_builder(
    cli_db_url: Option<String>,
    cli_server_port: Option<u16>,
    cli_origin_port: Option<u16>,
    cli_log_level: Option<String>,
) -> Result<ServerConfig> {
    let folder_path = "./config";

    if !Path::new(folder_path).exists() {
        fs::create_dir_all(folder_path).unwrap_or_else(|_| {
            eprintln!(
                "Failed to create {} folder. Try manually create it or restart the app",
                "config".bright_red()
            )
        });
    }

    let config_path = "./config/config.toml";
    if Path::new(config_path).exists() {
        println!(
            "\nFound a {} file, Loading the values from the file...",
            "config".bright_green()
        );
    } else {
        let conf_blue = "config".bright_green();
        println!("\nCouldnt find a {} file! Trying to construct one...\n((you can directly modify values from the {} file or passing them through Arguments when the you start application))\n",
        conf_blue,
        conf_blue);

        let default_config = ServerConfig::default();
        let toml_string = toml::to_string_pretty(&default_config)?;

        let mut file = fs::File::create(config_path)?;
        file.write_all(toml_string.as_bytes())?;

        println!("{} created successfuly!", "config.toml".bright_yellow());
    }

    let mut config = Config::builder().add_source(File::new(config_path, FileFormat::Toml));

    if let Some(db_url) = cli_db_url {
        config = config.set_override("db_url", db_url)?;
    }
    if let Some(server_port) = cli_server_port {
        config = config.set_override("server_port", server_port)?;
    }
    if let Some(origin_port) = cli_origin_port {
        config = config.set_override("origin_port", origin_port)?;
    }
    if let Some(log_level) = cli_log_level {
        config = config.set_override("log_level", log_level)?;
    }

    let result: ServerConfig = config.build()?.try_deserialize()?;

    let toml_string = toml::to_string_pretty(&result)?;
    fs::write(config_path, toml_string)?;
    Ok(result)
}
