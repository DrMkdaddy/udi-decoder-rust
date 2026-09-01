//! # FDA & EU MDR Medical Device UDI Decoder (GS1, HIBCC, ICCBBA) — Rust Client SDK
//!
//! Official client for FDA & EU MDR Medical Device UDI Decoder (GS1, HIBCC, ICCBBA) on RapidAPI.
//! Obtain your API key at: <https://rapidapi.com/noor-mkdad-apis-noor-mkdad-apis-default/api/fda-eu-mdr-medical-device-udi-decoder-gs1-hibcc-iccbba>

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::time::Duration;

/// Standard RapidAPI response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(default)]
    pub data: Option<T>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub subscribe_url: Option<String>,
}

/// Client configuration for FDA & EU MDR Medical Device UDI Decoder (GS1, HIBCC, ICCBBA).
#[derive(Debug, Clone)]
pub struct RapidApiConfig {
    pub api_key: String,
    pub base_url: String,
    pub rapidapi_host: String,
}

impl Default for RapidApiConfig {
    fn default() -> Self {
        Self {
            api_key: env::var("RAPIDAPI_KEY").unwrap_or_default(),
            base_url: "https://fda-eu-mdr-medical-device-udi-decoder-gs1-hibcc-iccbba.p.rapidapi.com".to_string(),
            rapidapi_host: "fda-eu-mdr-medical-device-udi-decoder-gs1-hibcc-iccbba.p.rapidapi.com".to_string(),
        }
    }
}

/// Main client for FDA & EU MDR Medical Device UDI Decoder (GS1, HIBCC, ICCBBA).
pub struct UdiDecoderClient {
    config: RapidApiConfig,
    client: Client,
}

impl UdiDecoderClient {
    /// Creates a new client instance with optional custom configuration.
    pub fn new(config: Option<RapidApiConfig>) -> Self {
        let config = config.unwrap_or_default();
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    /// Checks the health and status of the edge service.
    pub fn get_health(&self) -> Result<ApiResponse<Value>, reqwest::Error> {
        let url = format!("{}/health", self.config.base_url.trim_end_matches('/'));
        let res = self
            .client
            .get(&url)
            .header("x-rapidapi-key", &self.config.api_key)
            .header("x-rapidapi-host", &self.config.rapidapi_host)
            .send()?
            .json::<ApiResponse<Value>>()?;

        Ok(res)
    }

    /// Submits a payload to the validation endpoint.
    pub fn validate<P: Serialize>(&self, payload: &P) -> Result<ApiResponse<Value>, reqwest::Error> {
        let url = format!("{}/api/v1/validate", self.config.base_url.trim_end_matches('/'));
        let res = self
            .client
            .post(&url)
            .header("x-rapidapi-key", &self.config.api_key)
            .header("x-rapidapi-host", &self.config.rapidapi_host)
            .json(payload)
            .send()?
            .json::<ApiResponse<Value>>()?;

        Ok(res)
    }
}
