// JSON SERIALIZATION MODULES
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// STANDARD IO MODULES
use std::fs::{create_dir_all, File};
use std::io::Write;

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub function: String,
    pub data: Value,
}

#[tauri::command]
pub fn database_handler(payload: Payload) -> Result<Value, Value> {
    match payload.function.as_str() {
        "test_handler" => Ok(test_handler(payload.data)?),
        "configure_database_path" => Ok(configure_database_path(payload.data)?),
        _ => Err(json!({ "error": "Unknown function", "function": payload.function })),
    }
}

pub fn test_handler(packet: Value) -> Result<Value, Value> {
    // This function is a placeholder for testing purposes.
    // It currently returns a success message.
    Ok(json!({
        "message": "Test handler executed successfully",
    }))
}

pub fn configure_database_path(packet: Value) -> Result<Value, Value> {
    let location = packet
        .as_str()
        .ok_or_else(
            || json!({ "error": "Invalid location format", "function": "configure_database_path" }),
        )?
        .to_string();

    // 🤔 VALIDATE THE LOCATION STRING
    if location.is_empty() {
        return Err(json!({
            "error": "Location cannot be empty",
            "function": "configure_database_path"
        }));
    }

    // 💾 CACHE FOLDER CREATION PATHS
    let configuration_path = format!("{}/configuration", location);
    let database_path = format!("{}/database", location);

    println!(
        "\n⚙️  Configuration Path: {}",
        configuration_path.to_string()
    );
    println!("💿 Database Path: {}\n", database_path.to_string());

    // 📁 CREATE FOLDER USING PATHS
    create_dir_all(&configuration_path).map_err(|e| e.to_string())?;
    create_dir_all(&database_path).map_err(|e| e.to_string())?;

    // 📦 DEFINE JSON CONTENTS WITH DATABASE LOCATION
    let json_contents = format!(
        r#"{{ 
        "DatabaseLocation": "{database_path}",
        "AppState": {{
            "isDatabaseConfigured": true
        }} 
    }}"#
    );

    // CREATE CONFIGURATION FILE PATH
    let configuration_file_path = format!("{configuration_path}/configuration.json");

    // CREATE NEW FILE HANDLER/INSTANCE "IN-MEMORY"
    let mut file = File::create(&configuration_file_path).map_err(|e| e.to_string())?;

    // WRITE JSON CONTENTS TO THE FILE ON DISK
    file.write_all(json_contents.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "status": "SUCCESS",
        "configuration_file_path": configuration_file_path,
        "database_path": database_path,
    }))
}
