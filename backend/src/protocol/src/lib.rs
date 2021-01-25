use std::fmt::Display;
use std::fmt::Write;

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    lat: f64,
    lon: f64,
    timezone: String,
    timezone_offset: f64,
    pub hourly: Vec<Hourly>,
}

#[derive(Debug, Deserialize)]
pub struct Hourly {
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
    pub snow: Option<Snow>,
    weather: Vec<Weather>,
    pop: f64,
}

#[derive(Debug, Deserialize)]
pub struct Snow {
    #[serde(rename(deserialize = "1h"))]
    pub hour: f64,
}
#[derive(Debug, Deserialize)]
pub struct Rain {
    #[serde(rename(deserialize = "1h"))]
    pub hour: f64,
}
#[derive(Debug, Deserialize)]
pub struct Weather {
    id: f64,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Serialize)]
pub struct SnowFall {
    location: String,
    total: f64,
    hourly: Vec<Accumulation>,
}

impl SnowFall {
    pub fn new(location: String, total: f64, hourly: Vec<Accumulation>) -> Self {
        Self {
            location,
            total,
            hourly,
        }
    }
}

impl Display for SnowFall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now = Utc::now();

        let mut res = String::from("[\n\t\t");
        for (hour_offset, snowfall) in &self.hourly {
            if res != String::from("[\n\t\t") {
                write!(&mut res, "\n\t\t")?;
            }
            let offset = Duration::hours(hour_offset.clone() as i64);
            let future_date = now + offset;
            write!(&mut res, "{} {}mm", future_date.to_rfc2822(), snowfall)?;
        }
        write!(&mut res, "\n\t]")?;

        if res == String::from("[\n\t\t\n\t]") {
            res = String::from("None");
        }

        write!(
            f,
            r#"
        Total Snowfall for {}: {} inches
        Hourly Snowfall for {}: {}
        "#,
            self.location, self.total, self.location, res
        )
    }
}

pub type Coords = (f64, f64); // Longitude, Latitude
pub type Accumulation = (u32, f64); // Hourly Offset, Volume
