// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::fs::{ self, File };
// use std::io::{ BufWriter, Error, Read, Write };

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] PngDecoding(#[from] png::DecodingError),
    #[error(transparent)] PngEncoding(#[from] png::EncodingError),
    #[error(transparent)] Io(#[from] std::io::Error),
}

//manually implementation seder::Serialize
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

fn png_path_out_edit(file_in_raw: String) -> String {
    let file_in = file_in_raw.replace("\\", "/");
    let last_slash_index = match file_in.rfind('/') {
        Some(index) => index,
        None => {
            println!("Error: Unable to find the directory path!");
            return String::new();
        }
    };

    // Extract directory path and filename
    let (dir_path, file_name) = file_in.split_at(last_slash_index + 1);

    // Find the last occurrence of '.' to extract the file extension
    let last_dot_index = match file_name.rfind('.') {
        Some(index) => index,
        None => {
            println!("Error: Unable to find the file extension!");
            return String::new();
        }
    };

    // Extract filename without extension
    let (file_name_without_ext, file_ext) = file_name.split_at(last_dot_index);

    // Build the new filename with "_edited" appended before the file extension
    let new_file_name = format!("{}{}{}", file_name_without_ext, "_edited", file_ext);

    // Concatanate the directory path and the new filename
    let file_out = format!("{}{}", dir_path, new_file_name);

    println!("This is the path for the modified png: {}", file_out);

    return file_out;
}

#[tauri::command]
fn png_metadata_edit(
    parameters: String,
    data_generation: String,
    file_path: String
) -> Result<(), Error> {
    println!("FILE PATH ===> {}", file_path);

    let decoder = png::Decoder::new(File::open(file_path.clone()).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    let png_info = reader.info();

    // Encode
    let path_out_string = png_path_out_edit(file_path);
    let path_out = Path::new(&path_out_string);
    let file = File::create(path_out)?;
    let ref mut w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, png_info.width, png_info.height);
    encoder.set_color(png_info.color_type);
    encoder.set_depth(png_info.bit_depth);
    encoder.add_text_chunk("parameters".to_string(), parameters);
    encoder.add_text_chunk("data_generation".to_string(), data_generation);
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
