#![allow(clippy::multiple_crate_versions)]

use config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Default error type for rot
pub struct RotError {
    /// The error message
    pub message: String,
}

impl From<String> for RotError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&str> for RotError {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

/// Configuration options for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOptions {
    /// The domain of the blog, e.g. "example.com"
    pub domain: String,
    /// The port for the server to listen on
    pub port: u16,
    /// Path to the directory containing the templates
    pub template_dir: String,
    /// Path to the directory containing the standard mdx posts
    pub post_dir: String,
}

impl TryFrom<HashMap<String, String>> for ConfigOptions {
    type Error = RotError;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let domain = value.get("domain").ok_or("domain not found")?.clone();
        let port = value
            .get("port")
            .ok_or("8080")?
            .parse()
            .map_err(|_| "port is not a number")?;
        let template_dir = value
            .get("template_dir")
            .ok_or("template_dir not found")?
            .clone();
        let post_dir = value.get("post_dir").ok_or("post_dir not found")?.clone();

        Ok(Self {
            domain,
            port,
            template_dir,
            post_dir,
        })
    }
}

/// Retrieves the configuration from the default configuration file.
///
/// # Errors
///
/// Returns a `RotError` if:
/// - The configuration file cannot be deserialized
/// - Required configuration fields are missing or invalid
pub fn get_config() -> Result<ConfigOptions, RotError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("/etc/rot/Config"))
        .build()
        .unwrap_or_default();

    let map: HashMap<String, String> = settings
        .try_deserialize()
        .map_err(|e| RotError::from(format!("Error deserializing config: {e}")))?;

    map.try_into()
}
