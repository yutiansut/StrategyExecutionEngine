/*******************************************************************************
 Copyright (c) 2024.

 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:

 The above copyright notice and this permission notice shall be included in
 all copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 THE SOFTWARE.
 ******************************************************************************/

/******************************************************************************
    Author:
    Email: jb@taunais.com
    Date: 24/5/24
 ******************************************************************************/

use serde_json::json;
use std::env;
use thiserror::Error;
use crate::constants::{*};

/// Represents the configuration options for the application.
#[derive(Debug, Clone)]
pub struct Config {
    /// The WebSocket URL for connecting to the server.
    pub kafka_url: String,
    pub nats_url: String,
}

/// An enum representing various errors that can occur during configuration.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Error indicating that a required environment variable is missing.
    #[error("missing environment variable: {0}")]
    MissingEnvVar(String),
}

impl Config {
    /// Creates a new `Config` instance by reading environment variables.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if a required environment variable is missing.
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Config {
            kafka_url: Self::get_env_var_or_default("KAFKA_URL", KAFKA_URL.to_string()),
            nats_url: Self::get_env_var_or_error("NATS_URL")?,
        })
    }

    /// Gets the value of an environment variable or returns a default value if the variable is not set.
    ///
    /// # Arguments
    ///
    /// * `var_name` - The name of the environment variable.
    /// * `default_value` - The default value to use if the environment variable is not set.
    fn get_env_var_or_default(var_name: &str, default_value: String) -> String {
        env::var(var_name).unwrap_or(default_value)
    }

    /// Gets the value of an environment variable or returns an error if the variable is not set.
    ///
    /// # Arguments
    ///
    /// * `var_name` - The name of the environment variable.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError::MissingEnvVar` if the environment variable is not set.
    fn get_env_var_or_error(var_name: &str) -> Result<String, ConfigError> {
        env::var(var_name).map_err(|_| ConfigError::MissingEnvVar(var_name.to_string()))
    }

    /// Serializes the configuration to a JSON string.
    ///
    /// # Returns
    ///
    /// Returns a JSON string representation of the configuration.
    ///
    /// # Errors
    ///
    /// Returns a `serde_json::Error` if serialization fails.
    pub fn print_as_json(&self) -> serde_json::Result<String> {
        let json_config = json!({
            "KAFKA_URL": self.kafka_url,
            "NATS_URL": self.nats_url,
        });
        serde_json::to_string_pretty(&json_config)
    }
}
