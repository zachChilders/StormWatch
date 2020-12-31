use serde::Deserialize;

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
