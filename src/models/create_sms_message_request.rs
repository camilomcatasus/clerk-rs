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
pub struct CreateSmsMessageRequest {
	/// The message you would like to send
	#[serde(
		rename = "message",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub message: Option<Option<String>>,
	/// The ID of a verified phone number the SMS message should be sent to
	#[serde(rename = "phone_number_id", skip_serializing_if = "Option::is_none")]
	pub phone_number_id: Option<String>,
}

impl CreateSmsMessageRequest {
	pub fn new() -> CreateSmsMessageRequest {
		CreateSmsMessageRequest {
			message: None,
			phone_number_id: None,
		}
	}
}
