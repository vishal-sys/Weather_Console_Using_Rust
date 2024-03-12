use reqwest;
use serde::Deserialize;
use tokio;
use chrono::{TimeZone, Utc};

#[derive(Deserialize)]
struct WeatherData {
    main : Main,
    weather : Vec<Weather>,
    name : String,
    sys : Sys,
    wind : Wind,
    clouds : Clouds,
}

#[derive(Deserialize)]
struct Main {
    temp : f64,
    feels_like : f64,
    humidity : i32,
    pressure : i32,
}

#[derive(Deserialize)]
struct Weather {
    description : String,
}

#[derive(Deserialize)]
struct Sys {
    country : String,
    sunrise : i64,
    sunset : i64,
}

#[derive(Deserialize)]
struct Wind {
    speed : f64,
    deg : i32,
}

#[derive(Deserialize)]
struct Clouds {
    all : i32,
}

// Define an asynchronous function to fetch weather data for a given city
async fn fetch_weather(api_key : &str , city : String) -> Result<(), reqwest::Error>{

    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city.trim(),
                                api_key);

    let response = reqwest::get(url).await?;

    if response.status().is_success() {

        let weather_data : WeatherData = response.json().await?;

        let temperature = weather_data.main.temp;
        let feels_like = weather_data.main.feels_like;
        let humidity = weather_data.main.humidity;
        let pressure = weather_data.main.pressure;

        let description = &weather_data.weather[0].description;
        let location = &weather_data.name;
        let country = &weather_data.sys.country;
        let sunrise = Utc.timestamp(weather_data.sys.sunrise as i64, 0);
        let sunset = Utc.timestamp(weather_data.sys.sunset as i64, 0);
        let wind_speed = weather_data.wind.speed;
        let wind_deg = weather_data.wind.deg;
        let cloud_cover = weather_data.clouds.all;

        let ist_sunrise = sunrise.with_timezone(&chrono::FixedOffset::east(5 * 3600));
        let ist_sunset = sunset.with_timezone(&chrono::FixedOffset::east(5 * 3600));

        println!("Weather in city: {}, {:.2}°C (feels like {:.2}°C), {}% humidity, {} hPa, {}% cloud cover", location.trim(), temperature-273.15, feels_like-273.15, humidity, pressure, cloud_cover);
        println!("Country: {}, Sunrise: {}, Sunset: {}", country, ist_sunrise.format("%H:%M"), ist_sunset.format("%H:%M"));
        println!("Wind: {} m/s, {}°", wind_speed, wind_deg);

    }
    else {
        println!("Error : {}", response.status());
    }
    Ok(())

}

#[tokio::main]
async fn main() {
    let api_key = "<Your  API>";
    let mut city = String::new();
    println!("Enter the location:-");

    std::io::stdin().read_line(&mut city).expect("Failed to read the line");

    tokio::spawn(fetch_weather(api_key, city));

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
 
}