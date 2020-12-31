use eyre::Result;

use weather_protocol::*;

type Coords = (f64, f64, &'static str);

const MAMMOTH: Coords = (37.6482765, -118.9832411, "Mammoth");
const RENO: Coords = (39.5197729, -119.9283731, "Reno");
const SLT: Coords = (38.9227921, -120.013534, "South Lake");

fn get_weather(coordinates: Coords) -> Result<()> {
    let key = "115310efdc47011d9ab0ca9a2ea1fb23";

    let api_uri = format!(
        "https://api.openweathermap.org/data/2.5/onecall?lat={lat}&lon={long}&exclude=minutely,current,daily&appid={key}&units=imperial",
        lat = coordinates.0,
        long = coordinates.1,
        key = key
    );

    let body = reqwest::blocking::get(&api_uri)?.text()?;
    let response: WeatherResponse = serde_json::from_str(&body)?;

    let hourly = response.hourly;

    const MM_IN_INCHES: f64 = 25.4;
    let mut total_snowfall = 0.0;
    for (i, hour) in hourly.iter().enumerate() {
        if let Some(snow) = &hour.snow {
            println!(
                "It's going to snow {mm}mm in {i} hours",
                mm = snow.hour,
                i = i
            );
            total_snowfall += snow.hour;
        }
    }
    println!(
        "Total snowfall for the next 48 hours in {}: {:?} inches",
        coordinates.2,
        total_snowfall / MM_IN_INCHES
    );

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn weather() {
        get_weather(MAMMOTH).unwrap();
        println!("==========================");
        get_weather(RENO).unwrap();
        println!("==========================");
        get_weather(SLT).unwrap();
    }
}
