pub struct GarminClient {
    pub client: reqwest::Client,
    pub username: String,
    pub password: String,
    pub session_id: String,
    pub jwt: String,
    pub bearer: String,
}

pub async fn login(garmin: &mut GarminClient) {
    println!("SESSIONID={}", garmin.session_id);

    // TODO: retrieve the bearer
    garmin.password = "".to_string(); // avoid having the password loaded more than the strictly required
}
