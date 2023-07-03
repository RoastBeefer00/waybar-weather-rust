use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref WEATHER_CODES: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("113", "󰖨");
        map.insert("116", "⛅️");
        map.insert("119", "☁️");
        map.insert("122", "☁️");
        map.insert("143", "🌫");
        map.insert("176", "🌦");
        map.insert("179", "🌧");
        map.insert("182", "🌧");
        map.insert("185", "🌧");
        map.insert("200", "⛈");
        map.insert("227", "🌨");
        map.insert("230", "❄️");
        map.insert("248", "🌫");
        map.insert("260", "🌫");
        map.insert("263", "🌦");
        map.insert("266", "🌦");
        map.insert("281", "🌧");
        map.insert("284", "🌧");
        map.insert("293", "🌦");
        map.insert("296", "🌦");
        map.insert("299", "🌧");
        map.insert("302", "🌧");
        map.insert("305", "🌧");
        map.insert("308", "🌧");
        map.insert("311", "🌧");
        map.insert("314", "🌧");
        map.insert("317", "🌧");
        map.insert("320", "🌨");
        map.insert("323", "🌨");
        map.insert("326", "🌨");
        map.insert("329", "❄️");
        map.insert("332", "❄️");
        map.insert("335", "❄️");
        map.insert("338", "❄️");
        map.insert("350", "🌧");
        map.insert("353", "🌦");
        map.insert("356", "🌧");
        map.insert("359", "🌧");
        map.insert("362", "🌧");
        map.insert("365", "🌧");
        map.insert("368", "🌨");
        map.insert("371", "❄️");
        map.insert("374", "🌧");
        map.insert("377", "🌧");
        map.insert("386", "⛈");
        map.insert("389", "🌩");
        map.insert("392", "⛈");
        map.insert("395", "❄️");
        
        map
    };
}

pub fn get_weather_icon(code: &str) -> String {
    let icon = WEATHER_CODES.get(code).unwrap();
    String::from(icon.to_owned())
}

