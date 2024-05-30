use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fs;

pub trait Store {
    fn set_changelog(&mut self, locale: &str, version: &str, changelog: &str)
        -> Result<(), String>;
}

pub struct AppStore {
    pub key_path: String,
    pub issuer_id: String,
    token: Option<String>,
    token_expiration: Option<usize>,
    app_id: String,
}

impl AppStore {
    pub fn new(key_path: String, issuer_id: String, app_id: String) -> Self {
        AppStore {
            key_path,
            issuer_id,
            app_id,
            token: None,
            token_expiration: None,
        }
    }

    fn token(&mut self) -> Option<String> {
        if !self.is_logged_in() {
            match self.login() {
                Ok(_) => {
                    return self.token.clone();
                }

                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        return self.token.clone();
    }

    fn login(&mut self) -> Result<(), String> {
        let key_id = self
            .key_path
            .split('_')
            .nth(1)
            .and_then(|s| s.split('.').nth(0));

        if key_id.is_none() {
            return Err("Invalid key path name. It should be AuthKey_{ID}.p8".to_string());
        }

        let key_id = key_id.unwrap_or_default();

        println!("Logging in to App Store: {}", key_id);

        let key_string = fs::read_to_string(&self.key_path).map_err(|e| e.to_string())?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        let expiration = now + 60 * 5;

        let claims = Claims {
            iss: self.issuer_id.clone(),
            iat: now,
            exp: expiration,
            aud: "appstoreconnect-v1".to_string(),
        };

        let token = encode(
            &Header {
                alg: jsonwebtoken::Algorithm::ES256,
                typ: Some("JWT".to_string()),
                kid: Some(key_id.to_string()),
                ..Default::default()
            },
            &claims,
            &EncodingKey::from_ec_pem(&key_string.as_str().as_bytes()).unwrap(),
        )
        .map_err(|e| e.to_string())?;

        self.token_expiration = Some(expiration);
        self.token = Some(token);

        Ok(())
    }

    fn is_logged_in(&self) -> bool {
        let is_expired: bool = self
            .token_expiration
            .map(|exp| {
                exp < std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs() as usize
            })
            .unwrap_or(true);

        return self.token.is_some() && !is_expired;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    iat: usize,
    exp: usize,
    aud: String,
}

const APP_STORE_CONNECT_URL: &str = "https://api.appstoreconnect.apple.com/v1";

fn ep(endpoint: &str) -> String {
    format!("{}/{}", APP_STORE_CONNECT_URL, endpoint)
}

#[derive(Debug, Serialize, Deserialize)]
struct AppStoreMultiResponse<T> {
    data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppStoreSingleResponse<T> {
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppStoreVersionData {
    #[serde(rename = "type")]
    _type: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppStoreVersionLocalizationAttributes {
    locale: String,
    #[serde(rename = "whatsNew")]
    whats_new: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppStoreVersionLocalizationAttributesPatch {
    #[serde(rename = "whatsNew")]
    whats_new: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppStoreVersionLocalizationData<T> {
    #[serde(rename = "type")]
    _type: String,
    id: String,
    attributes: T,
}

type AppStoreVersionResponse = AppStoreMultiResponse<AppStoreVersionData>;
type AppStoreVersionLocalizationResponse =
    AppStoreMultiResponse<AppStoreVersionLocalizationData<AppStoreVersionLocalizationAttributes>>;
type AppStoreVersionLocalizationResponsePatch = AppStoreSingleResponse<
    AppStoreVersionLocalizationData<AppStoreVersionLocalizationAttributesPatch>,
>;

struct AppStoreDataSource {
    token: String,
}

impl AppStoreDataSource {
    fn new(token: String) -> Self {
        AppStoreDataSource { token }
    }

    fn get_app_version_localizations(
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

    fn patch_whats_new(&self, localization_id: &str, whats_new: &str) -> Result<(), String> {
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

    fn get_app_store_version(
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
}

impl Store for AppStore {
    fn set_changelog(
        &mut self,
        locale: &str,
        version: &str,
        changelog: &str,
    ) -> Result<(), String> {
        let token = self.token().ok_or("Not logged in".to_string())?;
        let data_source = AppStoreDataSource::new(token);
        let response = data_source.get_app_store_version(&self.app_id, &version);

        if let Ok(response) = response {
            let has_only_one_element = response.data.len() == 1;

            if !has_only_one_element {
                return Err("More than one version found matching the version name".to_string());
            }

            let version_id = response.data[0].id.clone();

            let localizations = data_source
                .get_app_version_localizations(&version_id)
                .map_err(|e| e.to_string())?;

            let localization = localizations.data.iter().find(|l| {
                l.attributes.locale.to_lowercase() == locale.to_lowercase() || locale.is_empty()
            });

            if let Some(localization) = localization {
                data_source
                    .patch_whats_new(&localization.id, changelog)
                    .map_err(|e| e.to_string())?;
            } else {
                return Err("Localization not found".to_string());
            }

            return Ok(());
        } else {
            return Err(response.unwrap_err().to_string());
        }
    }
}
