use std::sync::Arc;

use warp::Filter;

use weather::OpenWeatherClient;

struct Context {
    weather_client: OpenWeatherClient,
}

impl Context {
    fn new() -> Self {
        Self {
            weather_client: OpenWeatherClient::new(),
        }
    }
}

async fn weather(
    param: String,
    station_context: Arc<Context>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let snowfall = station_context
        .clone()
        .weather_client
        .get_snowfall(param)
        .await
        .unwrap();

    Ok(warp::reply::json(&snowfall))
}

#[tokio::main]
async fn main() {
    let station_context = Arc::new(Context::new());

    let weather = warp::path("weather")
        .and(warp::path::param())
        .and_then(move |param| weather(param, station_context.clone()));

    warp::serve(weather).run(([0, 0, 0, 0], 3030)).await;
}
