use html2md;
use std::fmt;

/// Errors that occur during html conversion to markdown
pub enum HtmlConversionError {
    /// Empty string passed as HTML
    EmptyHtml,
}

impl fmt::Display for HtmlConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HtmlConversionError::EmptyHtml => {
                write!(f, "Empty string passed as `html`.")
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
/// <https://crates.io/crates/html2md>
pub fn convert_html_to_markdown(html: String) -> Result<String, HtmlConversionError> {
    if html.is_empty() {
        return Err(HtmlConversionError::EmptyHtml);
    }

    return Ok(html2md::parse_html(&html));
}

/// `markdown` module unit tests
#[cfg(test)]
mod tests {
    use super::*;

    /// Empty html string
    #[test]
    fn empty_html_returns_early_with_error() {
        let result: String = convert_html_to_markdown("".to_string())
            .unwrap_err()
            .to_string();

        let expected_error: String = "Empty string passed as `html`.".to_string();

        assert_eq!(result, expected_error);
    }

    #[test]
    /// Empty file name string
    fn converts_valid_html_to_valid_markdown() {
        let result: String = match convert_html_to_markdown("<h1>Header!</h1>".to_string()) {
            Ok(markdown) => markdown,
            Err(_) => "Failed!".to_string(),
        };

        let expected_markdown: String = "Header!\n==========".to_string();

        assert_eq!(result, expected_markdown);
    }
}
