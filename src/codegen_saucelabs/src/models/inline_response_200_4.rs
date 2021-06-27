/*
 * Sauce Labs REST API
 *
 * This is a REST API documentation provided by Sauce Labs
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: support@saucelabs.com
 * Generated by: https://openapi-generator.tech
 */

/// InlineResponse2004 : desired capabilities of manual job



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2004 {
    /// system browser name (e.g. \"chrome\")
    #[serde(rename = "browser")]
    pub browser: String,
    #[serde(rename = "members")]
    pub members: Vec<String>,
    /// system OS name (e.g. \"Windows 2012 R2\")
    #[serde(rename = "os")]
    pub os: String,
    /// owner of manual job
    #[serde(rename = "owner")]
    pub owner: String,
    /// browser resolution (e.g. \"1400x1050\")
    #[serde(rename = "resolution")]
    pub resolution: String,
    /// tunnel id to run manual test on (if available)
    #[serde(rename = "tunnel", skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<serde_json::Value>,
    /// start url of manual job
    #[serde(rename = "url")]
    pub url: String,
    /// system browser version
    #[serde(rename = "version")]
    pub version: String,
}

impl InlineResponse2004 {
    /// desired capabilities of manual job
    pub fn new(browser: String, members: Vec<String>, os: String, owner: String, resolution: String, url: String, version: String) -> InlineResponse2004 {
        InlineResponse2004 {
            browser,
            members,
            os,
            owner,
            resolution,
            tunnel: None,
            url,
            version,
        }
    }
}


