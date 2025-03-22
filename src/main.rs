// In src/main.rs
use ratatui_templates::app::AppResult;
use ratatui_templates::connection::{get_data, CityInfo};

#[tokio::main]
async fn main() -> AppResult<()> {
    println!("=======================================================");
    println!("           WEATHER INFORMATION FOR ROMANIA             ");
    println!("=======================================================");

    let cities = vec!["Bucharest", "Craiova", "Timisoara", "Iasi", "Constanta", "Brasov", "Cluj-Napoca", "Galati", "Ploiesti", "Oradea"];

    for city in cities {
        println!("Fetching weather data for {}...", city);

        match get_data(city.to_string()).await {
            Ok(info) => {
                println!("{}, {} - Current Weather", info.name, info.country);
                println!("  Timezone: UTC{:+}", info.timezone / 3600);
                println!("  Sunrise: {}", info.sunrise.format("%H:%M:%S"));
                println!("  Sunset: {}", info.sunset.format("%H:%M:%S"));
                println!("  Weather: {}", info.weather);
                println!("  Temperature: {}°C", info.temperature);
                println!("  Humidity: {}%", info.humidity);
                println!("  Wind Speed: {} m/s", info.wind_speed);
                println!("  Pressure: {} hPa", info.pressure);
                println!("  Visibility: {} meters", info.visibility);
                println!("  Description: {}", info.description);
                println!("-------------------------------------------------------");
            },
            Err(e) => {
                println!("Error getting data for {}: {}", city, e);
                println!("-------------------------------------------------------");
            }
        }
    }

    println!("Weather information retrieved successfully!");
    Ok(())
}
