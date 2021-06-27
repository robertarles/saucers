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
pub struct OrgTeamConcurrencyValues {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<Box<crate::models::ConcurrencyValues>>,
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Box<crate::models::ConcurrencyValues>>,
}

impl OrgTeamConcurrencyValues {
    pub fn new() -> OrgTeamConcurrencyValues {
        OrgTeamConcurrencyValues {
            current: None,
            allowed: None,
        }
    }
}


