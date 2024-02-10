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
    #[error(transparent)] Io(#[from] std::io::Error),
}

//manually implementation seder::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// use img_parts::png::Png;
// use img_parts::{ ImageEXIF, ImageICC };
// use std::fs::{ self, File };
// use std::fmt;

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
    println!("INFO LIST --> {:?}", &reader.info().uncompressed_latin1_text[1]);
    // let img_info = format!(
    //     "parameters:{:?}, data_parameters:{:?}",
    //     &reader.info().uncompressed_latin1_text[0].text,
    //     &reader.info().uncompressed_latin1_text[1].text
    // );
    // formatted_strings.push_str(&img_info);
    let parameters = format!("{:?}", &reader.info().uncompressed_latin1_text[0].text);
    let data_parameters = format!("{:?}", &reader.info().uncompressed_latin1_text[1].text);
    formatted_strings.push(parameters);
    formatted_strings.push(data_parameters);
    // for text_chunk in &reader.info().uncompressed_latin1_text {
    //     println!("Keyword: {:?}", text_chunk.keyword);
    //     println!("Text Chunk: {:?}", text_chunk);
    // }
    Ok(formatted_strings)
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, read_img_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
