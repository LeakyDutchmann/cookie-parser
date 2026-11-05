use std::{fs::{self, File}, io::self};
use std::io::prelude::*;
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
            println!("ERROR: {e}");
        }
    }

    println!("The end of program. Please press enter to exit");
    
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        break;
    }
}

fn handle() -> anyhow::Result<()> {
    println!("Reading input file");
    
    match std::fs::create_dir_all("source") {
        Ok(_) => {},
        Err(e) => {
            return Err(anyhow!("Failed to create dir at source/base64.txt Error {e}"));
        }
    }

    println!(
        "source exists: {}",
        std::path::Path::new("source").exists()
    );

    let mut content = String::new();
    if let Ok(mut file) = File::open("source/base64.txt") {
        match file.read_to_string(&mut content) {
            Ok(_) => {},
            Err(e) => {return Err(anyhow!("Failed to read input file: {e}"));}
        }
    } else {
        let line = "Paste you base64 here!".as_bytes();
        match fs::write("source/base64.txt", line) {
            Ok(_) => {},
            Err(e) => {
                return Err(anyhow!("failed to initialize base64.txt. Error: {e}"));
            }
        }
    }
    
    
    
    let input = content.trim();

    if input.is_empty() {
        return Err(anyhow!("Input file is empty. Error."))
    }
    
    let decoded = match general_purpose::STANDARD.decode(input) {
        Ok(b) => {
            println!("Succssesfully decoded bytes from base64");
            b
        }
        Err(e) => {
            return Err(anyhow!("Failed to decode bytes from base64. Error: {e}"));
        }
    };


    println!("Deserializing into proper internal format");
    
    let cookies: Vec<Cookie> = match serde_json::from_slice(&decoded) {
        Ok(v) => {
            v
        }
        Err(e) => {
            return Err(anyhow!("Failed to deserialize data into json structure: {e}"));
        }
    };

    println!("Deserialized: {cookies:?}");

    let path = "json/".to_string();
    
    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {},
        Err(e) => {
            return Err(anyhow!("Failed to create dir at {path}. Error {e}"));
        }
    }

    let json = match serde_json::to_string_pretty(&cookies) {
        Ok(js) => js,
        Err(e) => {
            return Err(anyhow!("Failed to serialize structure into json. Error: {e}"));
        }
    };

    match fs::write(path.clone() + "cookies.json", json) {
        Ok(_) => {
            println!("SUCCESS: File is saved to json/coookies.json")
        }, 
        Err(e) =>{
            return Err(anyhow!("Failed to write file at {path}. Error: {e}"));
        }
    }
    Ok(())
}