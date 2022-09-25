#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::PathBuf, str::FromStr, fs::File, collections::HashMap};

use serde::{Serialize, Deserialize};
use tauri::{Runtime, App, AppHandle, api::dialog::FileDialogBuilder, Window};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn ss_to_vulnus_local<R:Runtime>(wind:Window<R>,paths: HashMap<String,String>,export_to:PathBuf,meta: vconvert::vulnus::MetaData) -> Result<(),ConvertError> {

    
    
    paths.iter().map(|(path,diff)| -> Result<(),ConvertError> {
        let f = std::fs::read_to_string(path).or_else(|x| Err(ConvertError::IoError(x.to_string())))?;
        let ss_map = vconvert::soundspace::Map::from_str(&f).or_else(|x| Err(ConvertError::ParseError(x.to_string())))?;
        let vul_map : vconvert::vulnus::Map = ss_map.into();

        let map_f = File::create(export_to.join(format!("{}.json",diff))).or_else(|x| Err(ConvertError::IoError(x.to_string())))?;
        serde_json::to_writer(map_f, &vul_map).or_else(|x| Err(ConvertError::SerdeSerError(x.to_string())))?;
        Ok(())
    }).flatten().collect::<()>();


    let meta_f = File::create(export_to.join("meta.json")).or_else(|x| Err(ConvertError::IoError(x.to_string())))?;

    serde_json::to_writer(meta_f, &meta).or_else(|x| Err(ConvertError::SerdeSerError(x.to_string())))?;

    Ok(())
}

#[derive(Debug,Serialize,Deserialize)]
enum ConvertError {
    IoError(String),
    ParseError(String),
    SerdeSerError(String),
    Unknown
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ss_to_vulnus_local])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


