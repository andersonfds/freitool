use crate::data::{
    app_store_datasource::AppStoreDataSource,
    google_play_datasource::{GooglePlayDataSource, Release, ReleaseNote, ReleaseStatus, Track},
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{fs, vec};

pub trait Store {
    fn set_changelog(&mut self, locale: &str, version: &str, changelog: &str)
        -> Result<(), String>;

    fn create_version(&mut self, version: &str) -> Result<(), String>;
}

pub struct AppStore {
    pub key_path: String,
    pub issuer_id: String,
    token: Option<String>,
    token_expiration: Option<usize>,
    app_id: String,
}

impl AppStore {
    pub fn new(
        key_path: Option<String>,
        issuer_id: Option<String>,
        app_id: Option<String>,
    ) -> Result<Self, String> {
        let key_path = key_path.ok_or("Key path is required")?;
        let issuer_id = issuer_id.ok_or("Issuer ID is required")?;
        let app_id = app_id.ok_or("App ID is required")?;

        return Ok(Self {
            key_path,
            issuer_id,
            app_id,
            token: None,
            token_expiration: None,
        });
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
            scope: None,
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

#[derive(Debug, Deserialize)]
struct ServiceAccount {
    client_email: String,
    private_key: String,
}

pub struct GooglePlay {
    pub key_path: String,
    pub track: String,
    token: Option<String>,
    package_name: String,
}

impl GooglePlay {
    pub fn new(key_path: Option<String>, package_name: Option<String>, track: Option<String>) -> Result<Self, String> {
        let key_path = key_path.ok_or("Key path is required")?;
        let package_name = package_name.ok_or("Package name is required")?;
        let track = track.ok_or("Track is required")?;

        return Ok(Self {
            key_path,
            token: None,
            package_name,
            track,
        });
    }

    fn get_private_token(&self) -> Result<String, String> {
        let service_account: ServiceAccount = serde_json::from_str(
            &std::fs::read_to_string(self.key_path.as_str()).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;
        let exp = now + 3600;

        let claims = Claims {
            iss: service_account.client_email,
            scope: Some("https://www.googleapis.com/auth/androidpublisher".to_string()),
            aud: "https://oauth2.googleapis.com/token".to_string(),
            iat: now,
            exp,
        };

        let token = encode(
            &Header {
                alg: jsonwebtoken::Algorithm::RS256,
                ..Default::default()
            },
            &claims,
            &EncodingKey::from_rsa_pem(service_account.private_key.as_bytes()).unwrap(),
        )
        .map_err(|e| e.to_string())?;

        Ok(token)
    }

    fn login(&mut self) -> Result<(), String> {
        if self.is_logged_in() {
            return Ok(());
        } else {
            let private_token = self.get_private_token()?;
            let signed_token = GooglePlayDataSource::get_signed_token(private_token.as_str())?;
            self.token = Some(signed_token);

            return Ok(());
        }
    }

    fn is_logged_in(&self) -> bool {
        return self.token.is_some();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    iat: usize,
    exp: usize,
    aud: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
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

    fn create_version(&mut self, version: &str) -> Result<(), String> {
        let token = self.token().ok_or("Not logged in".to_string())?;
        return AppStoreDataSource::create_version(token.as_str(), &self.app_id, version);
    }
}

impl Store for GooglePlay {
    fn set_changelog(
        &mut self,
        locale: &str,
        version: &str,
        changelog: &str,
    ) -> Result<(), String> {
        self.login()?;

        let track_name = &self.track;
        let token = self.token.as_ref().unwrap();

        let edit_id = GooglePlayDataSource::create_edit_session(token, &self.package_name)?;

        let track = GooglePlayDataSource::get_track(
            token,
            &self.package_name,
            edit_id.as_str(),
            track_name,
        )?;

        let release = track
            .releases
            .iter()
            .find(|r| r.status == ReleaseStatus::Draft && r.name == version)
            .ok_or("Release not found or in an uneditable state.")?;

        let track = Track {
            track: track_name.to_string(),
            releases: vec![Release {
                version_codes: None,
                status: release.status,
                name: release.name.clone(),
                release_notes: Some(vec![ReleaseNote {
                    language: locale.to_string(),
                    text: changelog.to_string(),
                }]),
            }],
        };

        GooglePlayDataSource::update_track(token, &self.package_name, edit_id.as_str(), track)?;
        GooglePlayDataSource::commit_edits(token, &self.package_name, &edit_id)?;

        return Ok(());
    }

    fn create_version(&mut self, version: &str) -> Result<(), String> {
        self.login()?;

        let token = self.token.as_ref().unwrap();
        let edit_id = GooglePlayDataSource::create_edit_session(token, &self.package_name)?;

        let current_track_data =
            GooglePlayDataSource::get_track(token, &self.package_name, &edit_id, &self.track)?;

        if current_track_data
            .releases
            .iter()
            .any(|r| r.name == version)
        {
            return Err("Version already exists".to_string());
        }

        let track = Track {
            track: self.track.clone(),
            releases: vec![Release {
                version_codes: None,
                status: ReleaseStatus::Draft,
                name: version.to_string(),
                release_notes: None,
            }],
        };

        GooglePlayDataSource::update_track(token, &self.package_name, edit_id.as_str(), track)?;
        GooglePlayDataSource::commit_edits(token, &self.package_name, &edit_id)?;

        return Ok(());
    }
}
