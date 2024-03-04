use html2md;
use std::fmt;

/// Errors that occur during html conversion markdown
pub enum HtmlConversionError {
    /// Empty string passed as HTML
    EmptyHtml,
}

impl fmt::Display for HtmlConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HtmlConversionError::EmptyHtml => {
                write!(f, "Emtpy string passed as `html`.")
            }
        }
    }
}

/// Converts an HTML formatted string to markdown.
///
/// # Arguments
/// `html: String` a string of HTML
///
/// # Reference
/// https://crates.io/crates/html2md
pub fn convert_html_to_markdown(html: String) -> Result<String, HtmlConversionError> {
    if html.is_empty() {
        return Err(HtmlConversionError::EmptyHtml);
    }

    return Ok(html2md::parse_html(&html));
}
