// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::fs::File;

#[path = "utils/utils.rs"]
mod utils;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] PngDecoding(#[from] png::DecodingError),
    #[error(transparent)] PngEncoding(#[from] png::EncodingError),
    #[error(transparent)] Io(#[from] std::io::Error),
}

//manually implementation serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn upload_img(file: String) -> Result<Vec<String>, Error> {
    let mut formatted_strings = Vec::new();
    let decoder = png::Decoder::new(std::fs::File::open(file.clone()).unwrap());
    let reader = decoder.read_info().unwrap();
    formatted_strings.push(file);
    if reader.info().uncompressed_latin1_text.len() > 0 {
        formatted_strings.push(reader.info().uncompressed_latin1_text[0].text.clone());
        if reader.info().uncompressed_latin1_text.len() > 1 {
            formatted_strings.push(reader.info().uncompressed_latin1_text[1].text.clone());
        } else {
            formatted_strings.push("no parameters extra data".to_string());
        }
    } else {
        println!("No elements exist");
    }
    Ok(formatted_strings)
}

#[tauri::command]
fn png_metadata_edit(
    parameters: String,
    data_generation: String,
    no_extra_data: bool,
    file_path: String
) -> Result<(), Error> {
    // Decode
    let decoder = png::Decoder::new(File::open(file_path.clone()).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    let png_info = reader.info();

    // Encode
    let path_out_string = utils::png_path_out_edit(file_path);
    let path_out = Path::new(&path_out_string);
    let file = File::create(path_out)?;
    let ref mut w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, png_info.width, png_info.height);
    encoder.set_color(png_info.color_type);
    encoder.set_depth(png_info.bit_depth);
    let _ = encoder.add_text_chunk("parameters".to_string(), parameters);
    if no_extra_data == true {
    } else {
        let _ = encoder.add_text_chunk("data_generation".to_string(), data_generation);
    }
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(bytes).unwrap();
    Ok(())
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![upload_img, png_metadata_edit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
