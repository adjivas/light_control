use super::Light;

use std::error::Error;

pub fn get(http_rest_host: &str, http_rest_pass: &str) -> Result<Light, Box<dyn Error>> {
    let client: reqwest::blocking::Client = reqwest::blocking::Client::new();
    let request = client.get(http_rest_host)
        .header("API-Key", http_rest_pass)
        .send()?;

    Ok(request.json()?)
}

pub fn patch(http_rest_host: &str, http_rest_pass: &str, message: String) -> Result<Light, Box<dyn Error>> {
    let client: reqwest::blocking::Client = reqwest::blocking::Client::new();
    let request = client.patch(http_rest_host)
        .header("API-Key", http_rest_pass)
        .body(message).send()?;

    Ok(request.json()?)
}
