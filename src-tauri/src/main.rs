// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Exif(#[from] exif::Error),
    #[error(transparent)] ImgParts(#[from] img_parts::Error),
    #[error(transparent)] Png(#[from] png::DecodingError),
    #[error(transparent)] Io(#[from] std::io::Error),
}

//manually implementation seder::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn read_img_test() -> Result<Vec<String>, Error> {
    let mut formatted_strings = Vec::new();
    let img_path =
        "C:/Users/wesle/OneDrive/Área de Trabalho/Projects/edit_png/src-tauri/src/test_img/to_test.png";
    let decoder = png::Decoder::new(std::fs::File::open(img_path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // println!("Reader: {}", reader);
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    println!("Info: {:?}", info);
    formatted_strings.push(reader.info().uncompressed_latin1_text[0].text.clone());
    formatted_strings.push(reader.info().uncompressed_latin1_text[1].text.clone());
    // for text_chunk in &reader.info().uncompressed_latin1_text {
    //     println!("Keyword: {:?}", text_chunk.keyword);
    //     println!("Text Chunk: {:?}", text_chunk);
    // }
    Ok(formatted_strings)
}

use tauri::http::Request;
use serde_json::Value;

#[tauri::command]
fn upload_img_test(file: String) -> Result<String, Error> {
    println!("THAT'S IT? {}", file);

    Ok("maybe?".to_string())
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, read_img_test, upload_img_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
