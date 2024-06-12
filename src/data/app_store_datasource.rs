use std::{any::Any, collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

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
        let client = reqwest::blocking::Client::new()
            .get(ep(format!(
                "apps/{}/appStoreVersions?filter[versionString]={}",
                app_id, version
            )
            .as_str()))
            .bearer_auth(self.token.clone());

        let response = client.send().map_err(|e| e.to_string())?;
        let response = response.text().map_err(|e| e.to_string())?;
        let response: AppStoreVersionResponse =
            serde_json::from_str(&response).map_err(|e| e.to_string())?;

        Ok(response)
    }

    pub fn create_version(&self, app_id: &str, version: &str) -> Result<(), String> {
        let mut attributes = HashMap::<String, Box<dyn Any>>::new();
        attributes.insert("platform".to_string(), Box::new("IOS"));
        attributes.insert("versionString".to_string(), Box::new(version.to_string()));

        let mut app = HashMap::<String, Box<dyn Any>>::new();
        app.insert("type".to_string(), Box::new("apps"));
        app.insert("id".to_string(), Box::new(app_id.to_string()));

        let mut relationships = HashMap::<String, Box<dyn Any>>::new();
        relationships.insert("app".to_string(), Box::new(app));

        let mut data = HashMap::<String, Box<dyn Any>>::new();
        data.insert("attributes".to_string(), Box::new(attributes));
        data.insert("relationships".to_string(), Box::new(relationships));
        data.insert("type".to_string(), Box::new("appStoreVersions"));

        let request_body: AppStoreSingleData<HashMap<String, Box<dyn Any>>> =
            AppStoreSingleData { data };

        return Ok(());
    }
}
