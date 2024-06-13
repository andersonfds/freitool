use super::datasource::ResponseMapper;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};

pub struct AppStoreDataSource {
    token: String,
}

const APP_STORE_CONNECT_URL: &str = "https://api.appstoreconnect.apple.com/v1";

fn ep(endpoint: &str) -> String {
    format!("{}/{}", APP_STORE_CONNECT_URL, endpoint)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStoreMultiData<T> {
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStoreSingleData<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStoreVersionData {
    #[serde(rename = "type")]
    _type: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStoreVersionLocalizationAttributes {
    pub locale: String,
    #[serde(rename = "whatsNew")]
    pub whats_new: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStoreVersionLocalizationAttributesPatch {
    #[serde(rename = "whatsNew")]
    pub whats_new: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStoreVersionLocalizationData<T> {
    #[serde(rename = "type")]
    _type: String,
    pub id: String,
    pub attributes: T,
}

pub type AppStoreVersionResponse = AppStoreMultiData<AppStoreVersionData>;
pub type AppStoreVersionLocalizationResponse =
    AppStoreMultiData<AppStoreVersionLocalizationData<AppStoreVersionLocalizationAttributes>>;
pub type AppStoreVersionLocalizationResponsePatch =
    AppStoreSingleData<AppStoreVersionLocalizationData<AppStoreVersionLocalizationAttributesPatch>>;

impl AppStoreDataSource {
    pub fn new(token: String) -> Self {
        AppStoreDataSource { token }
    }

    pub fn get_app_version_localizations(
        &self,
        version_id: &str,
    ) -> Result<AppStoreVersionLocalizationResponse, String> {
        let client = reqwest::blocking::Client::new()
            .get(ep(format!(
                "appStoreVersions/{}/appStoreVersionLocalizations",
                version_id
            )
            .as_str()))
            .bearer_auth(self.token.clone());

        let response = client.send().map_err(|e| e.to_string())?;
        let response = response.text().map_err(|e| e.to_string())?;

        let response: AppStoreVersionLocalizationResponse =
            serde_json::from_str(&response).map_err(|e| e.to_string())?;

        Ok(response)
    }

    pub fn patch_whats_new(&self, localization_id: &str, whats_new: &str) -> Result<(), String> {
        let body = AppStoreVersionLocalizationResponsePatch {
            data: AppStoreVersionLocalizationData {
                _type: "appStoreVersionLocalizations".to_string(),
                id: localization_id.to_string(),
                attributes: AppStoreVersionLocalizationAttributesPatch {
                    whats_new: whats_new.to_string(),
                },
            },
        };

        let body_str = serde_json::to_string(&body).map_err(|e| e.to_string())?;

        let client = reqwest::blocking::Client::new()
            .patch(ep(format!(
                "appStoreVersionLocalizations/{}",
                localization_id
            )
            .as_str()))
            .bearer_auth(self.token.clone())
            .header("Content-Type", "application/json")
            .body(body_str);

        let response = client.send().map_err(|e| e.to_string())?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(format!(
                "Error: {}",
                response.text().map_err(|e| e.to_string())?
            ));
        }
    }

    pub fn get_app_store_version(
        &self,
        app_id: &str,
        version: &str,
    ) -> Result<AppStoreVersionResponse, String> {
        return reqwest::blocking::Client::new()
            .get(ep(format!(
                "apps/{}/appStoreVersions?filter[versionString]={}",
                app_id, version
            )
            .as_str()))
            .bearer_auth(self.token.clone())
            .send()
            .res::<AppStoreVersionResponse>();
    }

    pub fn create_version(token: &str, app_id: &str, version: &str) -> Result<(), String> {
        let request_body = json!({
            "data": {
                "attributes": {
                    "platform": "IOS",
                    "versionString": version,
                },
                "relationships": {
                    "app": {
                        "data": {
                            "id": app_id,
                            "type": "apps"
                        }
                    }
                },
                "type": "appStoreVersions",
            },
        });

        println!("{}", serde_json::to_string(&request_body).unwrap());

        return reqwest::blocking::Client::new()
            .post(ep("appStoreVersions"))
            .bearer_auth(token)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&request_body).map_err(|e| e.to_string())?)
            .send()
            .res::<Map<_, _>>()
            .map(|_| ())
            .map_err(|e| e.to_string());
    }
}
