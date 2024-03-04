use reqwest;
use std::fmt;

/// Errors possible in the HTML extraction process
#[derive(Debug)]
pub enum HtmlExtractionError {
    /// Empty URL string
    EmptyUrl,
    /// Invalid URL string
    InvalidUrl,
    /// Failure making GET request
    RequestFailure(reqwest::Error),
    /// Failure extracting body from response
    BodyExtractionFailure(reqwest::Error),
}

impl fmt::Display for HtmlExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HtmlExtractionError::EmptyUrl => write!(f, "Emtpy string passed as `url`."),
            HtmlExtractionError::InvalidUrl => write!(f, "Invalid URL."),
            HtmlExtractionError::RequestFailure(ref err) => err.fmt(f),
            HtmlExtractionError::BodyExtractionFailure(ref err) => err.fmt(f),
        }
    }
}

/// Makes a GET request and returns its body as a String.
///
/// # Arguments
/// `url: String` URL of the target webpage
pub fn extract_html_from_webpage(url: String) -> Result<String, HtmlExtractionError> {
    if url.is_empty() {
        return Err(HtmlExtractionError::EmptyUrl);
    }

    if !url.starts_with("http") || !url.starts_with("https") {
        return Err(HtmlExtractionError::InvalidUrl);
    }

    let response: reqwest::blocking::Response = match reqwest::blocking::get(url) {
        Ok(response) => response,
        Err(http_error) => return Err(HtmlExtractionError::RequestFailure(http_error)),
    };

    match response.text() {
        Ok(html) => return Ok(html),
        Err(body_extract_failure) => {
            return Err(HtmlExtractionError::BodyExtractionFailure(
                body_extract_failure,
            ))
        }
    };
}
