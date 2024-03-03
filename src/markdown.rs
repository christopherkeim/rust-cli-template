use html2md;

/// Converts an HTML formatted string to markdown.
///
/// # Arguments
/// `html: String` a string of HTML
///
/// # Reference
/// https://crates.io/crates/html2md
pub fn convert_html_to_markdown(html: String) -> String {
    return html2md::parse_html(&html);
}
