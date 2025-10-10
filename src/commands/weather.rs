use crate::{Context, Error};
use poise::CreateReply;
use reqwest;
use serde_json;
use serenity::all::{Colour, CreateEmbed, Timestamp};
use std::env;

/// What's the weather there?
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Where do you want to check the weather?"] location: String,
) -> Result<(), Error> {
    let key: String = env::var("WEATHER_API_KEY").expect("Expected a key for appId");
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={location}&appid={key}&units=metric"
    );

    let resp = match reqwest::get(url).await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    let json: serde_json::Value = serde_json::from_str(&resp).expect("file should be proper JSON");

    if json["cod"] == "404" {
        let not_found_embed: CreateEmbed = CreateEmbed::new()
            .description("Location not found.")
            .color(Colour::RED);

        ctx.send(CreateReply::default().embed(not_found_embed))
            .await?;
        return Ok(());
    }

    let weather_embed: CreateEmbed = CreateEmbed::new()
        .title(format!("Weather in {location}"))
        .url(format!("https://openweathermap.org/city/{}", json["id"]))
        .description(format!(
            "{} ({})",
            json["weather"][0]["main"].to_string().replace("\"", ""),
            json["weather"][0]["description"]
                .to_string()
                .replace("\"", "")
        ))
        .thumbnail(format!(
            "https://openweathermap.org/img/wn/{}@2x.png",
            json["weather"][0]["icon"].to_string().replace("\"", "")
        ))
        .fields([
            ("Temp", format!("{}°C", json["main"]["temp"]), true),
            ("Min", format!("{}°C", json["main"]["temp_min"]), true),
            ("Max", format!("{}°C", json["main"]["temp_max"]), true),
            (
                "Feels Like",
                format!("{}°C", json["main"]["feels_like"]),
                true,
            ),
            ("Humidity", format!("{}%", json["main"]["humidity"]), true),
            ("Wind Speed", format!("{}m/s", json["wind"]["speed"]), true),
        ])
        .colour(Colour::DARK_BLUE)
        .timestamp(Timestamp::now());

    ctx.defer_ephemeral().await?;

    ctx.send(CreateReply::default().embed(weather_embed))
        .await?;
    Ok(())
}
