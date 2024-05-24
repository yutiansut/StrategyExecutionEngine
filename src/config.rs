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

use serde::Serialize;
use serde_json::json;
use serde_json::Result as JsonResult;
use std::env;
use thiserror::Error;

/// Trait for serializing structures to JSON.
pub trait JsonSerializable {
    fn print_as_json(&self) -> JsonResult<String>;
}

/// Represents the configuration options for Kafka.
#[derive(Debug, Clone, Serialize)]
pub struct KafkaConfig {
    pub kafka_url: String,
}

/// Represents the configuration options for NATS.
#[derive(Debug, Clone, Serialize)]
pub struct NatsConfig {
    pub nats_url: String,
}

/// Represents the configuration options for RabbitMQ.
#[derive(Debug, Clone, Serialize)]
pub struct RabbitMqConfig {
    pub rabbitmq_url: String,
}

/// Represents the configuration options for ZeroMQ.
#[derive(Debug, Clone, Serialize)]
pub struct ZeroMqConfig {
    pub zmq_url: String,
}

/// Represents the configuration options for Redis.
#[derive(Debug, Clone, Serialize)]
pub struct RedisConfig {
    pub redis_url: String,
}

// Implementing JsonSerializable trait for each messaging configuration structure.

impl JsonSerializable for KafkaConfig {
     fn print_as_json(&self) -> JsonResult<String> {
        serde_json::to_string_pretty(self)
    }
}

impl JsonSerializable for NatsConfig {
    fn print_as_json(&self) -> JsonResult<String> {
        serde_json::to_string_pretty(self)
    }
}

impl JsonSerializable for RabbitMqConfig {
    fn print_as_json(&self) -> JsonResult<String> {
        serde_json::to_string_pretty(self)
    }
}

impl JsonSerializable for ZeroMqConfig {
    fn print_as_json(&self) -> JsonResult<String> {
        serde_json::to_string_pretty(self)
    }
}

impl JsonSerializable for RedisConfig {
    fn print_as_json(&self) -> JsonResult<String> {
        serde_json::to_string_pretty(self)
    }
}

/// Represents the general configuration options for the application.
#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub kafka: Option<KafkaConfig>,
    pub nats: Option<NatsConfig>,
    pub rabbitmq: Option<RabbitMqConfig>,
    pub zeromq: Option<ZeroMqConfig>,
    pub redis: Option<RedisConfig>,
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
            kafka: Self::get_kafka_config(),
            nats: Self::get_nats_config()?,
            rabbitmq: Self::get_rabbitmq_config(),
            zeromq: Self::get_zeromq_config(),
            redis: Self::get_redis_config(),
        })
    }

    /// Gets the Kafka configuration from environment variables.
    fn get_kafka_config() -> Option<KafkaConfig> {
        env::var("KAFKA_URL").ok().map(|kafka_url| KafkaConfig { kafka_url })
    }

    /// Gets the NATS configuration from environment variables.
    fn get_nats_config() -> Result<Option<NatsConfig>, ConfigError> {
        match env::var("NATS_URL") {
            Ok(nats_url) => Ok(Some(NatsConfig { nats_url })),
            Err(_) => Ok(None),
        }
    }

    /// Gets the RabbitMQ configuration from environment variables.
    fn get_rabbitmq_config() -> Option<RabbitMqConfig> {
        env::var("RABBITMQ_URL").ok().map(|rabbitmq_url| RabbitMqConfig { rabbitmq_url })
    }

    /// Gets the ZeroMQ configuration from environment variables.
    fn get_zeromq_config() -> Option<ZeroMqConfig> {
        env::var("ZMQ_URL").ok().map(|zmq_url| ZeroMqConfig { zmq_url })
    }

    /// Gets the Redis configuration from environment variables.
    fn get_redis_config() -> Option<RedisConfig> {
        env::var("REDIS_URL").ok().map(|redis_url| RedisConfig { redis_url })
    }
}

// Implementing JsonSerializable trait for Config structure.
impl JsonSerializable for Config {
    fn print_as_json(&self) -> JsonResult<String> {
        let json_config = json!({
            "kafka": self.kafka.as_ref().map(|k| k.print_as_json().unwrap_or_else(|_| "Error serializing KafkaConfig".to_string())),
            "nats": self.nats.as_ref().map(|n| n.print_as_json().unwrap_or_else(|_| "Error serializing NatsConfig".to_string())),
            "rabbitmq": self.rabbitmq.as_ref().map(|r| r.print_as_json().unwrap_or_else(|_| "Error serializing RabbitMqConfig".to_string())),
            "zeromq": self.zeromq.as_ref().map(|z| z.print_as_json().unwrap_or_else(|_| "Error serializing ZeroMqConfig".to_string())),
            "redis": self.redis.as_ref().map(|r| r.print_as_json().unwrap_or_else(|_| "Error serializing RedisConfig".to_string())),
        });
        serde_json::to_string_pretty(&json_config)
    }
}
