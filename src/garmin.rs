use std::u8;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::client::GarminClient;

#[derive(Debug, Deserialize)]
pub struct ActivityType {
    #[serde(rename(deserialize = "typeKey"))]
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct Activity {
    #[serde(rename(deserialize = "activityId"))]
    pub id: u64,
    #[serde(rename(deserialize = "activityName"))]
    pub name: String,
    #[serde(rename(deserialize = "activityType"))]
    pub atype: ActivityType,

    pub distance: Option<f64>,
    #[serde(rename(deserialize = "movingDuration"))]
    pub duration: Option<f64>,

    #[serde(rename(deserialize = "averageSpeed"))]
    pub average_speed: Option<f64>,
    #[serde(rename(deserialize = "maxSpeed"))]
    pub max_speed: Option<f64>,
    #[serde(rename(deserialize = "avgRespirationRate"))]
    pub average_respiration_rate: Option<f64>,

    #[serde(rename(deserialize = "averageHR"))]
    pub average_hr: Option<f32>,
    #[serde(rename(deserialize = "maxHR"))]
    pub max_hr: Option<f32>,
    #[serde(rename(deserialize = "aerobicTrainingEffect"))]
    pub aerobic_training_effect: Option<f32>,
    #[serde(rename(deserialize = "anaerobicTrainingEffect"))]
    pub anaerobic_training_effect: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct RaceParams {
    pub search: String,
    pub start: i32,
    pub limit: i32,
}

pub async fn load_races(garmin: GarminClient, params: RaceParams) -> Result<Vec<Activity>, reqwest::Error> {
    let url = reqwest::Url::parse_with_params(
        "https://connect.garmin.com/activitylist-service/activities/search/activities",
        &[
            ("search", params.search),
            ("limit", params.limit.to_string()),
            ("start", params.start.to_string()),
            ("_", Utc::now().timestamp_millis().to_string()),
        ],
    );

    let results: Vec<Activity> = garmin
        .client
        .get(url.unwrap())
        .header("Cookie", format!("SESSIONID={}; JWT_FGP={}", garmin.session_id, garmin.jwt))
        .header("Authorization", format!("Bearer {}", garmin.bearer))
        .header("Accept", "application/json")
        .header("di-backend", "connectapi.garmin.com")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .send()
        .await?
        .json()
        .await?;

    let filtered_results: Vec<Activity> = results
        .into_iter()
        .filter(|activity| activity.atype.key == "rowing_v2")
        .collect();

    return Ok(filtered_results);
}
