use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::data::datasource::ResponseMapper;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseNote {
    pub language: String,
    pub text: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone, Copy)]
pub enum ReleaseStatus {
    #[serde(rename = "statusUnspecified")]
    Unspecified,

    #[serde(rename = "draft")]
    Draft,

    #[serde(rename = "inProgress")]
    InProgress,

    #[serde(rename = "halted")]
    Halted,

    #[serde(rename = "completed")]
    Completed,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    #[serde(rename = "versionCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_codes: Option<Vec<String>>,

    #[serde(rename = "releaseNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_notes: Option<Vec<ReleaseNote>>,

    pub status: ReleaseStatus,

    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub track: String,
    pub releases: Vec<Release>,
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

    pub fn update_track(
        token: &str,
        package_name: &str,
        edit_id: &str,
        track: Track,
    ) -> Result<(), String> {
        let endpoint = format!(
            "https://www.googleapis.com/androidpublisher/v3/applications/{}/edits/{}/tracks/{}",
            package_name, edit_id, track.track
        );
        let req_body = json!(track).to_string();

        return reqwest::blocking::Client::new()
            .put(endpoint)
            .bearer_auth(token)
            .header(ACCEPT, "application/json")
            .body(req_body)
            .send()
            .map(|_| ())
            .map_err(|e| format!("Failed to patch track: {}", e));
    }

    pub fn get_track(
        token: &str,
        package_name: &str,
        edit_id: &str,
        track: &str,
    ) -> Result<Track, String> {
        let endpoint = format!(
            "https://www.googleapis.com/androidpublisher/v3/applications/{}/edits/{}/tracks/{}",
            package_name, edit_id, track
        );

        return reqwest::blocking::Client::new()
            .get(endpoint)
            .bearer_auth(token)
            .send()
            .res::<Track>()
            .map_err(|e| format!("Failed to get tracks list: {}", e));
    }

    pub fn commit_edits(token: &str, package_name: &str, edit_id: &str) -> Result<(), String> {
        let endpoint = format!(
            "https://www.googleapis.com/androidpublisher/v3/applications/{}/edits/{}:commit",
            package_name, edit_id
        );

        return reqwest::blocking::Client::new()
            .post(endpoint)
            .bearer_auth(token)
            .header(ACCEPT, "application/json")
            .body("{}")
            .send()
            .map(|_| ())
            .map_err(|e| format!("Failed to commit edits: {}", e));
    }
}
