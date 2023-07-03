# waybar-weather-rust
This repo uses Rust to make a simple API call to `https://wttr.in` to get current weather information for Waybar.

## Customization
If you want to change the location of the weather, simply edit the URL of the REST API in `rest_api.rs`.  A good way to test this is to manually go to the website and play around with the location.

## Future Improvements
A cool way to improve this in the future would be to figure out your current location and get the weather based off that instead of a hard-coded location.

## Build
```bash
cargo build -r
cp ./target/release/weather /usr/local/bin
```
