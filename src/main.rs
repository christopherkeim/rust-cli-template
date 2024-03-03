mod cli;
mod files;
mod html;
mod markdown;

/// Entry point of the CLI application.
///
/// Parses input URL and filename (optional), makes a GET request
/// to that URL, converts the response HTML to markdown, and writes
/// the output file to this directory.
fn main() {
    let args: cli::Arguments = cli::get_cli_arguments();

    let html: String = match html::extract_html_from_webpage(args.url) {
        Ok(html) => html,
        Err(http_error) => {
            eprintln!("Error making GET request: {}", http_error.to_string());
            return;
        }
    };

    let markdown: String = markdown::convert_html_to_markdown(html);

    match files::write_markdown_file(markdown, &args.file_name) {
        Ok(_) => println!("Successfully created {}", args.file_name),
        Err(file_error) => {
            eprintln!("Error creating markdown file: {}", file_error.to_string());
        }
    };
}
