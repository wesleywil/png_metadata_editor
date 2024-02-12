// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Debug, thiserror::Error)]
enum Error {
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

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![upload_img])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
