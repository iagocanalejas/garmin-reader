use reqwest;

// bearer and jwt expire quite quickly
pub struct GarminClient {
    pub bearer: String,
    pub jwt: String,
}

pub async fn load_activities(garmin: &GarminClient) {
    let client = reqwest::Client::new();
    let res = client
        .get("https://connect.garmin.com/activitylist-service/activities/search/activities")
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
        .header("cookie", format!("JWT_FGP={}", garmin.jwt))
        .header("authorization", format!("Bearer {}", garmin.bearer))
        .header("di-backend", "connectapi.garmin.com")
        .send()
        .await
        .unwrap();

    println!(": {:?}", format!("JWT_FPG={}", garmin.jwt));
    println!(": {:?}", format!("Bearer {}", garmin.bearer));
    println!(": {:?}", res.status());
    println!(": {:?}", res.text().await.unwrap());
}
