use crate::{Context, Error};

#[derive(poise::ChoiceParameter, Debug)]
pub enum Animal {
    #[name = "Fox"]
    fox,
    #[name = "Cat"]
    cat,
    #[name = "Birb"]
    birb,
    #[name = "Panda"]
    panda,
    #[name = "Red Panda"]
    red_panda,
    #[name = "Raccoon"]
    racoon,
    #[name = "Koala"]
    koala,
    #[name = "Kangaroo"]
    kangaroo,
    #[name = "Whale"]
    whale,
    #[name = "Dog"]
    dog,
    #[name = "Bird"]
    bird,
}

/// Choose an animal, and get a fact and image about it.
#[poise::command(slash_command)]
pub async fn animal(
    ctx: Context<'_>,
    #[description = "Choose your favorite animal"] animal: Animal,
) -> Result<(), Error> {
    let url: String = format!("https://api.some-random-api.com/animal/{:?}", animal);

    let resp = match reqwest::get(url).await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    let json: serde_json::Value = serde_json::from_str(&resp).expect("file should be proper JSON");

    ctx.say(format!("{}", json["fact"].to_string().replace("\"", ""),))
        .await?;
    Ok(())
}

/// Spits out a random joke.
#[poise::command(slash_command)]
pub async fn joke(ctx: Context<'_>) -> Result<(), Error> {
    let url = "https://api.some-random-api.com/joke";

    let resp = match reqwest::get(url).await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    let json: serde_json::Value = serde_json::from_str(&resp).expect("file should be proper JSON");

    ctx.say(format!("{}", json["joke"].to_string().replace("\"", "")))
        .await?;
    Ok(())
}

#[derive(poise::ChoiceParameter, Debug)]
pub enum Animu {
    #[name = "nom"]
    nom,
    #[name = "poke"]
    poke,
    #[name = "cry"]
    cry,
    #[name = "kiss"]
    kiss,
    #[name = "pat"]
    pat,
    #[name = "hug"]
    hug,
    #[name = "wink"]
    wink,
}

/// Shows a random animu.
#[poise::command(slash_command)]
pub async fn animu(
    ctx: Context<'_>,
    #[description = "Choose your favorite animal"] animu: Animu,
) -> Result<(), Error> {
    let url: String = format!("https://api.some-random-api.com/animu/{:?}", animu);

    let resp = match reqwest::get(url).await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    let json: serde_json::Value = serde_json::from_str(&resp).expect("file should be proper JSON");

    ctx.say(format!("{}", json["link"].to_string().replace("\"", "")))
        .await?;
    Ok(())
}

/// Shows a random animu quote.
#[poise::command(slash_command)]
pub async fn animu_quote(ctx: Context<'_>) -> Result<(), Error> {
    let url = "https://api.some-random-api.com/animu/quote";

    let resp = match reqwest::get(url).await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    let json: serde_json::Value = serde_json::from_str(&resp).expect("file should be proper JSON");

    ctx.say(format!(
        "> {}\n{}\n-# {}",
        json["quote"].to_string().replace("\"", ""),
        json["name"].to_string().replace("\"", ""),
        json["anime"].to_string().replace("\"", " ")
    ))
    .await?;
    Ok(())
}
