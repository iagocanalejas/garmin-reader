mod client;
mod garmin;

use clap::Parser;
use dotenv::dotenv;
use garmin::RaceParams;

use crate::client::GarminClient;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let args = Cli::parse();
    let mut client = GarminClient {
        client: reqwest::Client::builder().build().unwrap(),
        username: std::env::var("GARMIN_USER").expect("GARMIN_USER must be set."),
        password: std::env::var("GARMIN_PASSWORD").expect("GARMIN_PASSWORD must be set."),
        session_id: std::env::var("GARMIN_SESSION").expect("GARMIN_SESSION must be set."),
        jwt: std::env::var("GARMIN_JWT").expect("GARMIN_JWT must be set."),
        bearer: std::env::var("GARMIN_BEARER").expect("GARMIN_BEARER must be set."),
    };

    client::login(&mut client).await;

    // TODO: iterate over all the races
    let races = garmin::load_races(
        client,
        RaceParams {
            search: "regata".to_string(),
            start: 0,
            limit: 100,
        },
    )
    .await;

    println!("{:?}", races);
}
