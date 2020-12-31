use std::{collections::HashMap, fmt::Display};

use std::fmt::Write;

use eyre::Result;

use weather_protocol::*;

const MAMMOTH: Coords = (37.6482765, -118.9832411);
const RENO: Coords = (39.5197729, -119.9283731);
const SLT: Coords = (38.9227921, -120.013534);

struct Endpoints {
    one_call: String,
}

struct Temperature {
    location: String,
    degrees: u64,
    feels_like: u64,
}

impl Endpoints {
    fn new() -> Self {
        Self {
            one_call: String::from("https://api.openweathermap.org/data/2.5/onecall?lat={lat}&lon={long}&exclude=minutely,current,daily&appid={key}&units=imperial"),
        }
    }
}

pub struct OpenWeatherClient {
    locations: HashMap<String, Coords>,
    api_key: String,
    endpoints: Endpoints,
}

impl OpenWeatherClient {
    pub fn new() -> Self {
        let api_key = String::from("115310efdc47011d9ab0ca9a2ea1fb23");

        let mut locations = HashMap::<String, Coords>::new();
        locations.insert(String::from("Mammoth"), MAMMOTH);
        locations.insert(String::from("Reno"), RENO);
        locations.insert(String::from("South Lake"), SLT);

        let endpoints = Endpoints::new();
        Self {
            locations,
            api_key,
            endpoints,
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

        const MM_IN_INCHES: f64 = 25.4;
        let mut total_snowfall = 0.0;
        let mut hourly_snowfall = Vec::<Accumulation>::new();
        for (i, hour) in hourly.iter().enumerate() {
            if let Some(snow) = &hour.snow {
                hourly_snowfall.push((i as u32, snow.hour));
                total_snowfall += snow.hour;
            }
        }

        Ok(SnowFall::new(location, total_snowfall, hourly_snowfall ))
    }
}
mod test {
    use super::*;

    #[tokio::test]
    async fn snowfall() {
        let client = OpenWeatherClient::new();
        let mammoth = client.get_snowfall(String::from("Mammoth")).await.unwrap();
        println!("{}", mammoth);

        let reno = client.get_snowfall(String::from("Reno")).await.unwrap();
        println!("{}", reno);

        let slt = client
            .get_snowfall(String::from("South Lake"))
            .await
            .unwrap();
        println!("{}", slt);
    }
}
