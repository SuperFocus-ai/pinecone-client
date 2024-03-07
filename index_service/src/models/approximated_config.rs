/*
 * defaultTitle
 *
 * defaultDescription
 *
 * The version of the OpenAPI document: 0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApproximatedConfig {
    #[serde(rename = "k_bits", skip_serializing_if = "Option::is_none")]
    pub k_bits: Option<i32>,
    #[serde(rename = "hybrid", skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<bool>,
}

impl ApproximatedConfig {
    pub fn new() -> ApproximatedConfig {
        ApproximatedConfig {
            k_bits: None,
            hybrid: None,
        }
    }
}


