//! Endpoints regarding users


#[doc(inline)]
pub use get_users::{GetUsers, GetUsersRequest};

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Gets information about one or more specified Twitch users.
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
pub mod get_users {
    use super::*;
    /// Query Parameters for [Get Users](super::get_users)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    pub struct GetUsersRequest {
        /// User ID. Multiple user IDs can be specified. Limit: 100.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        id: Vec<String>,
        /// User login name. Multiple login names can be specified. Limit: 100.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        login: Vec<String>,
    }

    /// Return Values for [Get Users](super::get_users)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    pub struct GetUsers {
        /// User’s broadcaster type: "partner", "affiliate", or "".
        pub broadcaster_type: Option<String>,
        /// User’s channel description.
        pub description: Option<String>,
        /// User’s display name.
        pub display_name: String,
        /// User’s email address. Returned if the request includes the [user:read:email scope](twitch_oauth2::Scope::UserReadEmail).
        pub email: Option<String>,
        /// User’s ID.
        pub id: String,
        /// User’s login name.
        pub login: String,
        /// URL of the user’s offline image.
        pub offline_image_url: Option<String>,
        /// URL of the user’s profile image.
        pub profile_image_url: Option<String>,
        /// User’s type: "staff", "admin", "global_mod", or "".
        #[serde(rename = "type")]
        pub type_: Option<String>,
        /// Total number of views of the user’s channel.
        pub view_count: usize,
    }

    impl helix::Request for GetUsersRequest {
        type Response = GetUsers;

        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
        const PATH: &'static str = "users";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];

        fn query(&self) -> Result<String, serde_urlencoded::ser::Error> {
            let mut s = String::new();
            s.push_str(&helix::repeat_query("id", self.id.as_slice()));
            if !s.is_empty() && !self.login.is_empty() {
                s.push_str("&")
            }
            s.push_str(&helix::repeat_query("login", self.login.as_slice()));
            Ok(s)
        }
    }

    impl helix::RequestGet for GetUsersRequest {}
}