use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    lat: f64,
    lon: f64,
    timezone: String,
    timezone_offset: f64,
    hourly: Vec<Hourly>,
}

#[derive(Debug, Deserialize)]
struct Hourly {
    dt: f64,
    temp: f64,
    feels_like: f64,
    pressure: f64,
    humidity: f64,
    dew_point: f64,
    uvi: f64,
    clouds: f64,
    visibility: f64,
    wind_speed: f64,
    wind_deg: f64,
    rain: Option<Rain>,
    snow: Option<Snow>,
    #[serde(skip_deserializing)]
    weather: Option<String>,
    pop: f64,
}

#[derive(Debug, Deserialize)]
struct Snow {
    #[serde(rename(deserialize = "1h"))]
    hour: f64,
}
#[derive(Debug, Deserialize)]
struct Rain {
    #[serde(rename(deserialize = "1h"))]
    hour: f64,
}

fn get_weather() -> Result<()> {
    let key = "115310efdc47011d9ab0ca9a2ea1fb23";

    let lat: f64 = 37.6482765;
    let long: f64 = -118.9832411;

    let uri = format!(
        "https://api.openweathermap.org/data/2.5/onecall?lat={lat}&lon={long}&exclude=minutely,current,daily&appid={key}&units=imperial",
        lat = lat,
        long = long,
        key = key
    );

    let body = reqwest::blocking::get(&uri)?.text()?;
    let response: WeatherResponse = serde_json::from_str(&body)?;

    let hourly = response.hourly;

    for (i, hour) in hourly.iter().enumerate() {
        if hour.snow.is_some() {
            println!("It's going to snow in {i} hours", i=i);
        }
    }

    Ok(())
}
fn main() -> Result<(), reqwest::Error> {
    get_weather().unwrap();

    Ok(())
}
