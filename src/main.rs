use chrono::Utc;
use exitfailure::ExitFailure;
use serde_json::{json, Value};
mod config;
mod structs;

use config::load_config;

const GITHUB_API_URL: &str = "https://api.github.com/user";
const WAKATIME_API_URL: &str = "https://wakatime.com/api/v1/users/current/summaries";

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    update_github_bio().await?;

    Ok(())
}

async fn update_github_bio() -> Result<(), ExitFailure> {
    let wakatime_stats: structs::WakatimeData = get_wakatime_total_time().await?;
    println!("{:?}", wakatime_stats.data[0].grand_total.text);

    let config = load_config()?;
    assert!(config.github_api_token.len() > 0);

    let client: reqwest::Client = reqwest::Client::new();

    let request_body: Value = json!({
        "bio":
            format!(
                "Testing stuff with Rust - {}",
                wakatime_stats.data[0].grand_total.text
            )
    });

    client
        .patch(GITHUB_API_URL)
        .header("Accept", "application/vnd.github.v3+json")
        .header(
            "Authorization",
            format!("token {}", config.github_api_token),
        )
        .header("User-Agent", "Awesome-Octocat-App")
        .json(&request_body)
        .send()
        .await?;

    Ok(())
}

async fn get_wakatime_total_time() -> Result<structs::WakatimeData, ExitFailure> {
    let config = load_config()?;
    assert!(config.wakatime_api_token.len() > 0);

    let url: String = format!(
        "{url}?api_key={api_key}&scope={scope}&start={start}&end={end}",
        url = WAKATIME_API_URL,
        api_key = config.wakatime_api_token,
        scope = "read_logged_time",
        start = Utc::now().to_string(),
        end = Utc::now().to_string()
    );

    let json: structs::WakatimeData = reqwest::get(url)
        .await?
        .json::<structs::WakatimeData>()
        .await?;

    Ok(json)
}
