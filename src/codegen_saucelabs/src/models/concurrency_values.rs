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
pub struct ConcurrencyValues {
    #[serde(rename = "mac_vms", skip_serializing_if = "Option::is_none")]
    pub mac_vms: Option<i32>,
    #[serde(rename = "rds", skip_serializing_if = "Option::is_none")]
    pub rds: Option<i32>,
    #[serde(rename = "vms", skip_serializing_if = "Option::is_none")]
    pub vms: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl ConcurrencyValues {
    pub fn new() -> ConcurrencyValues {
        ConcurrencyValues {
            mac_vms: None,
            rds: None,
            vms: None,
            id: None,
        }
    }
}


