# FDA & EU MDR Medical Device UDI Decoder (GS1, HIBCC, ICCBBA) — Rust Client

[![Crates.io](https://img.shields.io/crates/v/udi-decoder-client.svg)](https://crates.io/crates/udi-decoder-client)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![RapidAPI Listing](https://img.shields.io/badge/RapidAPI-Dedicated%20Listing-blueviolet)](https://rapidapi.com/noor-mkdad-apis-noor-mkdad-apis-default/api/fda-eu-mdr-medical-device-udi-decoder-gs1-hibcc-iccbba)

Official high-speed Rust client for **FDA & EU MDR Medical Device UDI Decoder (GS1, HIBCC, ICCBBA)**.

> Instant <5ms GS1-128, HIBCC Modulo-43 & ICCBBA ISBT 128 medical device barcode parser for FDA 21 CFR 801 & EU MDR compliance.

> 🔑 **Get your Dedicated API Key:** [Subscribe to FDA & EU MDR Medical Device UDI Decoder (GS1, HIBCC, ICCBBA) on RapidAPI](https://rapidapi.com/noor-mkdad-apis-noor-mkdad-apis-default/api/fda-eu-mdr-medical-device-udi-decoder-gs1-hibcc-iccbba)

---

## 🚀 Installation

```bash
cargo add udi-decoder-client
```

---

## ⚡ Quickstart

```rust
use udi_decoder_client::{UdiDecoderClient, RapidApiConfig};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UdiDecoderClient::new(Some(RapidApiConfig {
        api_key: "YOUR_RAPIDAPI_KEY".to_string(), // Get key from https://rapidapi.com/noor-mkdad-apis-noor-mkdad-apis-default/api/fda-eu-mdr-medical-device-udi-decoder-gs1-hibcc-iccbba
        ..Default::default()
    }));

    let result = client.validate(&json!({
        // Enter validation payload
    }))?;

    println!("Result: {:?}", result);
    Ok(())
}
```

---

## 🔗 Links
- 📖 [RapidAPI Documentation & Key](https://rapidapi.com/noor-mkdad-apis-noor-mkdad-apis-default/api/fda-eu-mdr-medical-device-udi-decoder-gs1-hibcc-iccbba)

## 📄 License
MIT © RapidAPI Microservices Portfolio
