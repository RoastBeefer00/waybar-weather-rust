mod codes;
mod rest_api;

use rest_api::get_weather_code;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    get_weather_code().await?;

    Ok(())
}
