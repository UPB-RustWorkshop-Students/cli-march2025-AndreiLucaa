use chrono::{DateTime, Local};
use dotenv::dotenv;
use std::env;


pub struct CityInfo {
    pub name: String,
    pub country: String,
    pub timezone: i32,
    pub sunrise: DateTime<Local>,
    pub sunset: DateTime<Local>,
    pub weather: String,
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub pressure: f64,
    pub visibility: f64,
    pub description: String,
    
}


pub async fn get_data(city: String) -> Result<CityInfo, String> {
    dotenv().ok();
    let city = city.trim();

    let api_key = env::var("API_KEY").expect("API_KEY not found in .env file");

    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {

                        parse_weather_data(&json, &city)
                    },
                    Err(e) => Err(format!("Failed to parse JSON: {}", e))
                }
            } else {
                Err(format!("API request failed with status: {}", response.status()))
            }
        },
        Err(error) => {
            Err(format!("Request error: {}", error))
        }
    }
}

fn parse_weather_data(data: &serde_json::Value, city_name: &str) -> Result<CityInfo, String> {
    // Extract relevant fields from JSON
    let country = data["sys"]["country"].as_str()
        .ok_or_else(|| "Country not found".to_string())?;

    let timezone = data["timezone"].as_i64()
        .ok_or_else(|| "Timezone not found".to_string())? as i32;

    let sunrise_timestamp = data["sys"]["sunrise"].as_i64()
        .ok_or_else(|| "Sunrise not found".to_string())?;
    let sunset_timestamp = data["sys"]["sunset"].as_i64()
        .ok_or_else(|| "Sunset not found".to_string())?;

    // Convert timestamps to DateTime<Local>
    let sunrise = chrono::NaiveDateTime::from_timestamp_opt(sunrise_timestamp, 0)
        .ok_or_else(|| "Invalid sunrise timestamp".to_string())
        .map(|ndt| chrono::DateTime::<chrono::Utc>::from_utc(ndt, chrono::Utc).with_timezone(&chrono::Local))?;

    let sunset = chrono::NaiveDateTime::from_timestamp_opt(sunset_timestamp, 0)
        .ok_or_else(|| "Invalid sunset timestamp".to_string())
        .map(|ndt| chrono::DateTime::<chrono::Utc>::from_utc(ndt, chrono::Utc).with_timezone(&chrono::Local))?;

    let weather = data["weather"][0]["main"].as_str()
        .ok_or_else(|| "Weather not found".to_string())?.to_string();

    let temperature = data["main"]["temp"].as_f64()
        .ok_or_else(|| "Temperature not found".to_string())?;

    let humidity = data["main"]["humidity"].as_f64()
        .ok_or_else(|| "Humidity not found".to_string())?;

    let wind_speed = data["wind"]["speed"].as_f64()
        .ok_or_else(|| "Wind speed not found".to_string())?;

    let pressure = data["main"]["pressure"].as_f64()
        .ok_or_else(|| "Pressure not found".to_string())?;

    let visibility = data["visibility"].as_f64()
        .ok_or_else(|| "Visibility not found".to_string())?;

    let description = data["weather"][0]["description"].as_str()
        .ok_or_else(|| "Description not found".to_string())?.to_string();

    Ok(CityInfo {
        name: city_name.to_string(),
        country: country.to_string(),
        timezone,
        sunrise,
        sunset,
        weather,
        temperature,
        humidity,
        wind_speed,
        pressure,
        visibility,
        description,
    })
}

