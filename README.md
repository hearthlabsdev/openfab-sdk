# Rust API client for OpenFAB

## Overview
This crate provides core models, and API calls used by the open fabrication protocol currently under heavy development.

## Creating an API Client
To interact with the OpenFAB API using the SDK, initialize a Configuration (API Client) instance with your API key, host URL, and custom user agent.

```rust
use openfab_sdk::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new("your-api-key-here")
        .host("http://localhost:7777")
        .user_agent("openfab-cli/0.1.0");

    // Use config for SDK operations...
    Ok(())
}
```

## Listing Resources
You can call listing methods directly through the configuration object. Some list operations support pagination options, those that don't are either expected to return only a few entries, or will have pagination in the future.

The Pagination is organized by timestamp descending.

```rust
use openfab_sdk::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new("your-api-key-here");

    // Fetch the first page of devices
    let page = Some(1);
    let devices = config.list_devices(page).await?;
    
    println!("{}", serde_json::to_string_pretty(&devices)?);

    // List all libraries (doesn't require pagination)
    let libraries = config.list_libraries().await?;
    println!("{}", serde_json::to_string_pretty(&libraries)?);

    Ok(())
}
```

## Fetching a Specific Resource
To retrieve details about a specific resource (such as a device by its unique ID):

```rust
use openfab_sdk::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new("your-api-key-here");

    let device_id = "device-uuid-1234";
    let device = config.get_device(device_id).await?;

    println!("{}", serde_json::to_string_pretty(&device)?);

    Ok(())
}
```

## Creating an Asset (With File Upload)
The SDK supports uploading files along with metadata body payloads (e.g., creating an asset inside a library).

```rust
use openfab_sdk::{Configuration, errors::SDKError, files::FileTypeChecker};
use std::path::PathBuf;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new("your-api-key-here");
    let file_checker = FileTypeChecker::default_checker();

    let path = PathBuf::from("./sample-asset.stl");
    
    // Detect mime type
    let mime_type = file_checker
        .detect_for_file(&path)
        .await
        .expect("failed to get file info")
        .mime;

    // Open file stream
    let file = File::open(&path).await?;
    
    // Define JSON body payload for the asset
    let asset_json = serde_json::json!({
        "name": "Sample 3D Model",
        "library_id": "lib-uuid-5678"
    });

    // Create the asset
    let response = config
        .create_asset(&asset_json, file, mime_type.to_string())
        .await?;

    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
```

## Command Line Interface
If users are interacting with the binary built from this crate, they can execute commands using structopt:

```rust
# List all connected devices
openfab-cli list devices --page=1

# Get details of a specific device
openfab-cli get devices --id=dev-999

# Create a new asset with a file attachment and body payload
openfab-cli create assets --file=model.stl --body='{"name":"Test Model"}'
```