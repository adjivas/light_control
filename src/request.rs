use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum State {
    #[default]
    ON,
    OFF,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Light {
    pub state: State,
    brightness: u8,
    white_value: u8,
    color: Color,
}

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

impl Light {
    pub fn is_powered_off(http_rest_host: &str, http_rest_pass: &str) -> Result<bool, Box<dyn Error>>  {
        match get(http_rest_host, http_rest_pass) {
            Ok(Light { state, .. }) => Ok(state == State::OFF),
            Err(why) => Err(why),
        }
    }

    pub fn power_on(http_rest_host: &str, http_rest_pass: &str) -> Result<Light, Box<dyn Error>> {
        let ref light: Light = Light {state: State::ON, brightness: u8::MAX, white_value: u8::MAX, ..Default::default()};

        let message = serde_json::to_string(light)?;
        patch(http_rest_host, http_rest_pass, message)
    }

    pub fn power_off(http_rest_host: &str, http_rest_pass: &str) -> Result<Light, Box<dyn Error>> {
        let ref light: Light = Light {state: State::OFF, ..Default::default()};

        let message = serde_json::to_string(light)?;
        patch(http_rest_host, http_rest_pass, message)
    }
}
