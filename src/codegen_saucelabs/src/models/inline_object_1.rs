/*
 * Sauce Labs REST API
 *
 * This is a REST API documentation provided by Sauce Labs
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: support@saucelabs.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject1 {
    /// if true it stores assets of the manual job
    #[serde(rename = "assets")]
    pub assets: bool,
    /// system browser name (e.g. \"chrome\")
    #[serde(rename = "browser")]
    pub browser: String,
    /// sanitized browser name displayed in the UI (e.g. \"Google Chrome\")
    #[serde(rename = "browserDisplay")]
    pub browser_display: String,
    /// sanitized browser version (e.g. \"53\")
    #[serde(rename = "browserVersionDisplay")]
    pub browser_version_display: String,
    /// device name if manual testing is on mobile
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// system OS name (e.g. \"Windows 2012 R2\")
    #[serde(rename = "os")]
    pub os: String,
    /// sanitized OS name (e.g. \"Windows\")
    #[serde(rename = "osDisplay")]
    pub os_display: String,
    /// browser resolution (e.g. \"1400x1050\")
    #[serde(rename = "res")]
    pub res: String,
    /// tunnel id to run manual test on (if available)
    #[serde(rename = "tunnel", skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<String>,
    /// url to navigate to
    #[serde(rename = "url")]
    pub url: String,
    /// system browser version
    #[serde(rename = "version")]
    pub version: String,
}

impl InlineObject1 {
    pub fn new(assets: bool, browser: String, browser_display: String, browser_version_display: String, os: String, os_display: String, res: String, url: String, version: String) -> InlineObject1 {
        InlineObject1 {
            assets,
            browser,
            browser_display,
            browser_version_display,
            device: None,
            os,
            os_display,
            res,
            tunnel: None,
            url,
            version,
        }
    }
}


