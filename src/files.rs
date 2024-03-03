use std::fs::File;
use std::io::Write;

/// Writes a markdown file to local storage.
///
/// # Arguments
/// `markdown: String` a string holding markdown
/// `file_name: String` name of output file
pub fn write_markdown_file(markdown: String, file_name: &str) -> Result<(), std::io::Error> {
    let mut md_file: File = match File::create(file_name) {
        Ok(md_file) => md_file,
        Err(file_creation_error) => return Err(file_creation_error),
    };

    match md_file.write_all(markdown.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(file_write_error) => return Err(file_write_error),
    };
}
