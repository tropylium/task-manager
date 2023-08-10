// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

use app::DbError;

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

trait ToErrorMessage {
  fn to_message(&self) -> String;
}

impl ToErrorMessage for DbError {
  fn to_message(&self) -> String {
    todo!()
  }
}
