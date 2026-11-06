use std::{fs::{self}, path::PathBuf};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::{general_purpose}};
use anyhow::anyhow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub domain: String,

    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<f64>,

    #[serde(rename = "hostOnly")]
    pub host_only: bool,

    #[serde(rename = "httpOnly")]
    pub http_only: bool,

    pub name: String,
    pub path: String,

    #[serde(rename = "sameSite")]
    pub same_site: Option<String>,

    pub secure: bool,
    pub session: bool,

    #[serde(rename = "storeId")]
    pub store_id: Option<String>,

    pub value: String,
}

fn main() {
    let result = handle();

    match result {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR: {e}\n");
        }
    }

    println!("The end of program. Please press enter to exit");
    
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

fn app_dir() -> anyhow::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent()
        .ok_or_else( || anyhow!("Failed to determine app's directory\n")
    )?;  
    Ok(dir.to_path_buf())
}

fn handle() -> anyhow::Result<()> {
    println!("Reading input file...\n");

    let base_dir = app_dir()?;

    let source_dir = base_dir.join("source");
    std::fs::create_dir_all(&source_dir)?;

    println!(
        "source exists: {}\n",
        source_dir.exists()
    );

    let base_64_file = source_dir.join("base64.txt");
    
    if !base_64_file.exists() {
        fs::write(&base_64_file, "This file was automatically created and opened by program. You can now paste code here and re-run the application.")?;
        println!("Input file: {:?}", &base_64_file);
        
        open::that(&base_64_file)?;
        return Ok(());
    }
    
    let content = fs::read_to_string(&base_64_file)?;
    let input = content.trim();

    if input.is_empty() {
        return Err(anyhow!("Input file is empty. Error.\n"))
    }
    
    let decoded = general_purpose::STANDARD.decode(input)?;
    
    println!("Succssesfully decoded {} bytes from base64\n", decoded.len());

    println!("Deserializing into proper internal format\n");
    
    let cookies: Vec<Cookie> = serde_json::from_slice(&decoded)?;

    println!("Deserialized {} cookies", cookies.len());

    let json_path = base_dir.join("json");
    std::fs::create_dir_all(&json_path)?;

    let json = serde_json::to_string_pretty(&cookies)?;

    let output_path = json_path.join("cookies.json");
    fs::write(&output_path, json)?;
    
    println!("SUCCESS: File saved to {}\n", output_path.display());
    Ok(())
}