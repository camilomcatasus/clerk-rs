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
pub struct OrganizationMembershipOrganization {
	#[serde(rename = "object")]
	pub object: Object,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "slug")]
	pub slug: String,
	#[serde(
		rename = "members_count",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub members_count: Option<Option<i32>>,
	#[serde(rename = "max_allowed_memberships")]
	pub max_allowed_memberships: i32,
	#[serde(rename = "public_metadata")]
	pub public_metadata: serde_json::Value,
	#[serde(rename = "private_metadata")]
	pub private_metadata: serde_json::Value,
	#[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
	pub created_by: Option<String>,
	/// Unix timestamp of creation.
	#[serde(rename = "created_at")]
	pub created_at: i64,
	/// Unix timestamp of last update.
	#[serde(rename = "updated_at")]
	pub updated_at: i64,
}

impl OrganizationMembershipOrganization {
	pub fn new(
		object: Object,
		id: String,
		name: String,
		slug: String,
		max_allowed_memberships: i32,
		public_metadata: serde_json::Value,
		private_metadata: serde_json::Value,
		created_at: i64,
		updated_at: i64,
	) -> OrganizationMembershipOrganization {
		OrganizationMembershipOrganization {
			object,
			id,
			name,
			slug,
			members_count: None,
			max_allowed_memberships,
			public_metadata,
			private_metadata,
			created_by: None,
			created_at,
			updated_at,
		}
	}
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "organization")]
	Organization,
}

impl Default for Object {
	fn default() -> Object {
		Self::Organization
	}
}
