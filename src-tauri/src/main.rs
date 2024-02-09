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

// fn read_img_test() -> Result<String, Error> {
//     let mut formatted_strings = String::new();
//     let file = std::fs::File::open(
//         "C:/Users/wesle/OneDrive/Área de Trabalho/Projects/edit_png/src-tauri/src/test_img/to_test.png"
//     )?;
//     let mut bufreader = std::io::BufReader::new(&file);
//     let exifreader = exif::Reader::new();
//     let exif = exifreader.read_from_container(&mut bufreader)?;
//     for f in exif.fields() {
//         let formatted_str = format!("All --> {:?}", f);
//         println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(&exif));
//         formatted_strings.push_str(&formatted_str);
//     }
//     Ok(formatted_strings)
// }
use std::fs::{ self, File };
use img_parts::png::Png;
use img_parts::{ ImageEXIF, ImageICC };
#[tauri::command]
fn read_img_test() -> Result<String, Error> {
    let mut formatted_strings = String::new();
    let path =
        "C:/Users/wesle/OneDrive/Imagens/AI_Generated/datasets/eva_green_dataset/output/to_send/to_edit/done/DftWuo86l-fCMaIh6WF2H_.png";
    let input = fs::read(&path)?;

    let png = Png::from_bytes(input.into())?;
    let iccprofile = png.icc_profile();
    let exif_metadata = png.exif();
    let png_encoded = png.encoder().read();
    let formatted_str = format!("All --> {:?}", exif_metadata);
    println!("{:?}", png_encoded);
    formatted_strings.push_str(&formatted_str);

    Ok(formatted_strings)
}
fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, read_img_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
