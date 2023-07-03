mod codes;
mod rest_api;

use rest_api::get_weather_code;

#[tokio::main]
async fn main() {
    get_weather_code().await;
}
