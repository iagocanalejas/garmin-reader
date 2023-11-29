use chrono::Utc;
use serde::Deserialize;

pub struct GarminClient {
    pub client: reqwest::Client,
    pub username: String,
    pub password: String,
    pub session_id: String,
    pub jwt: String,
    pub bearer: String,
}

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
    pub duration: f64,

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

// TODO: find races
// TODO: iterate over all the races
pub async fn load_races(garmin: GarminClient) {
    // "rowing_v2"
    let params = [
        ("search", "".to_string()),
        ("activityType", "rowing_v2".to_string()),
        ("limit", 20.to_string()),
        ("start", 0.to_string()),
        ("_", Utc::now().timestamp_millis().to_string()),
    ];
    let url = reqwest::Url::parse_with_params(
        "https://connect.garmin.com/activitylist-service/activities/search/activities",
        params,
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
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let filtered_results: Vec<&Activity> = results
        .iter()
        .filter(|activity| activity.atype.key == "rowing_v2")
        .collect();

    println!("{:?}", filtered_results);
}

pub async fn login(garmin: &mut GarminClient) {
    println!("SESSIONID={}", garmin.session_id);

    // TODO: retrieve the bearer
    garmin.password = "".to_string(); // avoid having the password loaded more than the strictly required
}
