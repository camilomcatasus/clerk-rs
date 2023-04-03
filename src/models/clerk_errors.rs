/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClerkErrors {
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::ClerkError>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl ClerkErrors {
    pub fn new(errors: Vec<crate::models::ClerkError>) -> ClerkErrors {
        ClerkErrors {
            errors,
            meta: None,
        }
    }
}

