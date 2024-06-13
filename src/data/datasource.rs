pub fn res<T>(response: Result<reqwest::blocking::Response, reqwest::Error>) -> Result<T, String>
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
            "ERROR: {}",
            response.text().map_err(|e| e.to_string())?
        ));
    }
}

pub trait ResponseMapper {
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
