use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct CaptchaFeature {
    pub enabled: bool,
    pub key: String
}

#[derive(Deserialize, Debug)]
pub struct Feature {
    pub enabled: bool,
    pub url: String
}


#[derive(Deserialize, Debug)]
pub struct VosoFeature {
    pub enabled: bool,
    pub url: String,
    pub ws: String
}

#[derive(Deserialize, Debug)]
pub struct RevoltFeatures {
    pub captcha: CaptchaFeature,
    pub email: bool,
    pub invite_only: bool,
    pub autumn: Feature,
    pub january: Feature,
    pub voso: VosoFeature
}
#[derive(Deserialize, Debug)]
pub struct RevoltConfig {
    pub revolt: String,
    pub features: RevoltFeatures,
    pub ws: String,
    pub app: String,
    pub vapid: String
}

#[derive(Debug)]
pub struct HTTPClient {
    pub config: RevoltConfig,
    pub token: String
}

impl HTTPClient {
    pub async fn new(api_url: String, token: String) -> Self {

        let config = reqwest::get(api_url.as_str())
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        HTTPClient {
            config,
            token
        }
    }
}
