mod garmin;
use clap::Parser;
use dotenv::dotenv;

use crate::garmin::GarminClient;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let args = Cli::parse();
    let client = GarminClient {
        bearer: std::env::var("GARMIN_BEARER").expect("GARMIN_BEARER must be set."),
        jwt: std::env::var("GARMIN_JWT").expect("GARMIN_JWT must be set."),
    };

    garmin::load_activities(&client).await;
}
