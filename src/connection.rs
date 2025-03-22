use chrono::{DateTime, Local};

struct CityInfo {
    name: String,
    country: String,
    timezone: i32,
    sunrise: DateTime<Local>,
    sunset: DateTime<Local>,


    // TODO: define elements in the structure
}

/// Method that is handling the request to the OpenWeather api
/// and parsing the response
///
/// Returns weather details about a certain city
pub fn get_data(city: String) -> Result<CityInfo, String> {
    let api_key = "d1e67bf495a01857c85d388d811707b1";
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    match reqwest::blocking::get(&url) {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>() {
                    Ok(json) => {
                        // Parse the response and create a CityInfo object
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
    let sunrise = chrono::DateTime::<chrono::Utc>::from_timestamp(sunrise_timestamp, 0)
        .ok_or_else(|| "Invalid sunrise timestamp".to_string())?
        .with_timezone(&chrono::Local);

    let sunset = chrono::DateTime::<chrono::Utc>::from_timestamp(sunset_timestamp, 0)
        .ok_or_else(|| "Invalid sunset timestamp".to_string())?
        .with_timezone(&chrono::Local);

    Ok(CityInfo {
        name: city_name.to_string(),
        country: country.to_string(),
        timezone,
        sunrise,
        sunset,
    })
}

//d1e67bf495a01857c85d388d811707b1
