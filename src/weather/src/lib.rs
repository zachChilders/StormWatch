use serde::Deserialize;
use std::{collections::HashMap, fs};

use eyre::Result;

use weather_protocol::*;

#[derive(Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
struct Config {
    api_key: String,
    locations: Vec<Location>,
}

pub struct OpenWeatherClient {
    locations: HashMap<String, Coords>,
    api_key: String,
}

impl OpenWeatherClient {
    pub fn new() -> Self {
        // Load config from file
        let config_file = fs::read_to_string("data.json").unwrap();
        let config: Config = serde_json::from_str(&config_file).unwrap();

        let mut locations = HashMap::<String, Coords>::new();
        for location in config.locations {
            let coords = (location.latitude, location.longitude);
            locations.insert(location.name, coords);
        }

        Self {
            locations,
            api_key: config.api_key,
        }
    }

    pub async fn get_snowfall(&self, location: String) -> Result<SnowFall> {
        let coordinates = self.locations[&location];
        let api_uri = format!(
        "https://api.openweathermap.org/data/2.5/onecall?lat={lat}&lon={long}&exclude=minutely,current,daily&appid={key}&units=imperial",
        lat = coordinates.0,
        long = coordinates.1,
        key = self.api_key
    );

        let body = reqwest::get(&api_uri).await?.text().await?;
        let response: WeatherResponse = serde_json::from_str(&body)?;

        let hourly = response.hourly;

        let mut total_snowfall = 0.0;
        let mut hourly_snowfall = Vec::<Accumulation>::new();
        for (i, hour) in hourly.iter().enumerate() {
            if let Some(snow) = &hour.snow {
                hourly_snowfall.push((i as u32, snow.hour));
                total_snowfall += snow.hour;
            }
        }

        Ok(SnowFall::new(location, total_snowfall, hourly_snowfall))
    }
}
mod test {

    #[tokio::test]
    async fn snowfall() {
        let client = crate::OpenWeatherClient::new();
        let mammoth = client.get_snowfall(String::from("Mammoth")).await.unwrap();
        println!("{}", mammoth);

        let reno = client.get_snowfall(String::from("Reno")).await.unwrap();
        println!("{}", reno);

        let slt = client
            .get_snowfall(String::from("SouthLake"))
            .await
            .unwrap();
        println!("{}", slt);
    }
}
