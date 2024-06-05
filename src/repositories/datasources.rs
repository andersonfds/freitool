use serde_json::json;

fn res<T>(response: Result<reqwest::blocking::Response, reqwest::Error>) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let response = response.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let body = response.text().map_err(|e| e.to_string())?;
        let json_body: serde_json::Value =
            serde_json::from_str(&body).map_err(|e| e.to_string())?;
        let result = serde_json::from_value::<T>(json_body).map_err(|e| e.to_string())?;
        return Ok(result);
    } else {
        return Err(format!(
            "Failed to deserialize response: {}",
            response.text().map_err(|e| e.to_string())?
        ));
    }
}

trait ResponseMapper {
    fn res<T>(self) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned;
}

impl ResponseMapper for Result<reqwest::blocking::Response, reqwest::Error> {
    fn res<T>(self) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
    {
        return res::<T>(self);
    }
}

pub struct GooglePlayDataSource {}

impl GooglePlayDataSource {
    pub fn get_signed_token(token: &str) -> Result<String, String> {
        let body_json = json!({
            "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
            "assertion": token,
        });

        return reqwest::blocking::Client::new()
            .post("https://oauth2.googleapis.com/token")
            .body(body_json.to_string())
            .send()
            .res::<serde_json::Value>()
            .map(|json_body| json_body["access_token"].as_str().unwrap().to_string())
            .map_err(|e| format!("Failed to get signed token: {}", e));
    }

    pub fn create_edit_session(token: &str, package_name: &str) -> Result<String, String> {
        let endpoint = format!(
            "https://www.googleapis.com/androidpublisher/v3/applications/{}/edits",
            package_name
        );

        return reqwest::blocking::Client::new()
            .post(endpoint)
            .bearer_auth(token)
            .body("{}")
            .send()
            .res::<serde_json::Value>()
            .map(|json_body| json_body["id"].as_str().unwrap().to_string())
            .map_err(|e| format!("Failed to create edit session: {}", e));
    }

    pub fn commit_edits(token: &str, package_name: &str, edit_id: &str) -> Result<(), String> {
        let endpoint = format!(
            "https://www.googleapis.com/androidpublisher/v3/applications/{}/edits/{}:commit",
            package_name, edit_id
        );

        return reqwest::blocking::Client::new()
            .post(endpoint)
            .bearer_auth(token)
            .body("{}")
            .send()
            .res::<serde_json::Value>()
            .map(|_| ())
            .map_err(|e| format!("Failed to commit edits: {}", e));
    }
}

pub struct AppStoreDataSource {}

impl AppStoreDataSource {
    pub fn new() -> Self {
        AppStoreDataSource {}
    }
}
