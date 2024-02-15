pub fn png_path_out_edit(file_in_raw: String) -> String {
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
