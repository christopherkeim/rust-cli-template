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

/// `html` module unit testing
#[cfg(test)]
mod tests {
    use super::*;

    /// Empty URL
    #[test]
    fn empty_url_returns_early_with_error() {
        let result = extract_html_from_webpage("".to_string())
            .unwrap_err()
            .to_string();

        let expected_error = "Emtpy string passed as `url`.".to_string();

        assert_eq!(result, expected_error);
    }

    #[test]
    /// Invalid URL
    fn invalid_url_returns_early_with_error() {
        let result = extract_html_from_webpage("invalid_webpage.com".to_string())
            .unwrap_err()
            .to_string();

        let expected_error = "Invalid URL.".to_string();

        assert_eq!(result, expected_error);
    }

    #[test]
    fn valid_request_returns_valid_html_string() {
        let result = match extract_html_from_webpage("https://www.google.com".to_string()) {
            Ok(html) => html,
            Err(_) => "Failed! {}".to_string(),
        };

        let expected_doctype = "<!doctype html>";

        assert!(result.starts_with(expected_doctype));
    }

    #[test]
    fn invalid_request_returns_request_failure() {
        let result =
            match extract_html_from_webpage("https://www.nonexistentwebsite.com/".to_string()) {
                Ok(html) => html,
                Err(request_failure) => request_failure.to_string(),
            };

        let expected_error_message =
            "error sending request for url (https://www.nonexistentwebsite.com/)";

        assert!(result.starts_with(expected_error_message));
    }
}
