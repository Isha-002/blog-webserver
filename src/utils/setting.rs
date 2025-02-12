use std::{fmt, fs, io::Write, path::Path, str::FromStr};
use anyhow::Result;
use config::{Config, File, FileFormat};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    debug,
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
            _ => Err(format!("Invalid log level: {}", s)),
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


#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub db_url: String,
    pub server_port: u16,
    pub origin_port: u16,
    pub log_level: LogLevel,
}

pub fn config_builder(args: ServerConfig) -> Result<ServerConfig> {
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
        println!("\nCouldnt find a {} file! Trying to constructing one...\n((you can directly modify values from the {} file or passing them through Arguments when the you start application))\n",
        conf_blue,
        conf_blue);

        let config_types = ServerConfig {
            db_url: args.db_url,
            server_port: args.server_port,
            origin_port: args.origin_port,
            log_level: args.log_level,
        };
        let toml_string =
            toml::to_string_pretty(&config_types)?;

        let mut file = fs::File::create(config_path)?;
        file.write_all(toml_string.as_bytes())?;

        println!("{} created successfuly!", "config.toml".bright_yellow());
    }

    let config = Config::builder()
        .add_source(File::new(config_path, FileFormat::Toml))
        .build()
        ?;

    let result: ServerConfig = config.try_deserialize()?;
    Ok(result)
}
