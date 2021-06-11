#![doc(alias = "channel.hype_train.end")]
//! A subscription to the specified channel expires.

use super::*;
/// [`channel.hype_train.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelSubscriptionEnd-beta): a subscription to the specified channel expires.
#[derive(Clone, Debug, typed_builder::TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscriptionEndV1 {
    /// The broadcaster user ID for the channel you want to get subscription end notifications for.
    #[builder(setter(into))]
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelSubscriptionEndV1 {
    type Payload = ChannelSubscriptionEndV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelSubscriptionEnd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::Scope::ChannelReadSubscriptions];
    const VERSION: &'static str = "1";
}

/// [`channel.hype_train.end`](ChannelSubscriptionEndV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelSubscriptionEndV1Payload {
    /// The user ID for the user whose subscription ended.
    pub broadcaster_user_id: types::UserId,
    /// The user login for the user whose subscription ended.
    pub broadcaster_user_login: types::DisplayName,
    /// The user display name for the user whose subscription ended.
    pub broadcaster_user_name: types::UserName,
    /// The broadcaster user ID.
    pub user_id: types::UserId,
    /// The broadcaster login.
    pub user_login: types::DisplayName,
    /// The broadcaster login.
    pub user_name: types::UserName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.subscription.end",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
               "broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "user_id": "1234",
            "user_login": "cool_user",
            "user_name": "Cool_User",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cooler_user",
            "broadcaster_user_name": "Cooler_User"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Payload::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}