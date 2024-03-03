use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(
    version = "0.1.0", 
    about = "A simple CLI application that converts a webpage to markdown.", 
    long_about = None
)]
pub struct Arguments {
    /// URL of webpage you'd like to convert
    #[arg(short = 'u', long = "url")]
    pub url: String,

    /// File you'd like to write
    #[arg(short = 'f', long = "file", default_value = "default.md")]
    pub file_name: String,
}

pub fn get_cli_arguments() -> Arguments {
    return Arguments::parse();
}
