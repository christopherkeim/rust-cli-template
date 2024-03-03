use reqwest;

/// Makes a GET request and returns its body as a String.
///
/// # Arguments
/// `url: String` URL of the target webpage
pub fn extract_html_from_webpage(url: String) -> Result<String, reqwest::Error> {
    let response: reqwest::blocking::Response = match reqwest::blocking::get(url) {
        Ok(response) => response,
        Err(http_error) => return Err(http_error),
    };

    match response.text() {
        Ok(html) => return Ok(html),
        Err(body_extract_failure) => return Err(body_extract_failure),
    };
}
