use serde::de::{Deserialize, Deserializer, Error};
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::types::*;

/// This object represents a Telegram user or bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: UserId,
    /// User‘s or bot’s first name.
    pub first_name: String,
    /// User‘s or bot’s last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// User‘s or bot’s username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// True, if this user is a bot.
    pub is_bot: bool,
    /// IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// This object represents a group.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Group {
    /// Unique identifier for this chat.
    pub id: GroupId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: bool,
    /// Invite link for this group, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
}

/// This object represents a supergroup.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Supergroup {
    /// Unique identifier for this chat.
    pub id: SupergroupId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Invite link for this supergroup, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
}

/// This object represents a channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Channel {
    /// Unique identifier for this chat.
    pub id: ChannelId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Invite link for this channel, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
}

/// This object represents a private, group or supergroup.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize)]
#[serde(tag = "type")]
pub enum MessageChat {
    #[serde(rename = "private")]
    Private(User),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "supergroup")]
    Supergroup(Supergroup),
    #[doc(hidden)]
    Unknown(RawChat),
}

impl MessageChat {
    pub fn id(&self) -> ChatId {
        match *self {
            MessageChat::Private(ref x) => x.id.into(),
            MessageChat::Group(ref x) => x.id.into(),
            MessageChat::Supergroup(ref x) => x.id.into(),
            MessageChat::Unknown(ref x) => x.id.into(),
        }
    }
}
//
// impl serde::ser::Serialize for MessageChat {
//     fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
//         S: Serializer {
//         match self {
//             MessageChat::Private(u) => {
//                 let raw_chat = RawChat {
//                     id: u.id.into(),
//                     type_: "private".to_string(),
//                     title: None,
//                     username: u.username.clone(),
//                     first_name: u.first_name.clone().into(),
//                     last_name: u.last_name.clone(),
//                     invite_link: None,
//                     language_code: u.language_code.clone(),
//                     all_members_are_administrators: None,
//                 };
//                 serializer.serialize_newtype_struct("chat", &raw_chat)
//             }
//             MessageChat::Group(g) => {
//                 let raw_chat = RawChat {
//                     id: g.id.into(),
//                     type_: "private".to_string(),
//                     title: g.title.clone().into(),
//                     username: None,
//                     first_name: None,
//                     last_name: None,
//                     invite_link: g.invite_link.clone(),
//                     language_code: None,
//                     all_members_are_administrators: g.all_members_are_administrators.into(),
//                 };
//                 serializer.serialize_newtype_struct("chat", &raw_chat)
//             }
//             MessageChat::Supergroup(g) => {
//                 let raw_chat = RawChat {
//                     id: g.id.into(),
//                     type_: "private".to_string(),
//                     title: g.title.clone().into(),
//                     username: None,
//                     first_name: None,
//                     last_name: None,
//                     invite_link: g.invite_link.clone(),
//                     language_code: None,
//                     all_members_are_administrators: None
//                 };
//                 serializer.serialize_newtype_struct("chat", &raw_chat)
//             }
//             MessageChat::Unknown(c) => {
//                 serializer.serialize_newtype_struct("chat", c)
//             }
//         }
//     }
// }

/// This object represents a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize)]
pub enum Chat {
    Private(User),
    Group(Group),
    Supergroup(Supergroup),
    Channel(Channel),
    #[doc(hidden)]
    Unknown(RawChat),
}

impl Chat {
    pub fn id(&self) -> ChatId {
        match *self {
            Chat::Private(ref x) => x.id.into(),
            Chat::Group(ref x) => x.id.into(),
            Chat::Supergroup(ref x) => x.id.into(),
            Chat::Channel(ref x) => x.id.into(),
            Chat::Unknown(ref x) => x.id.into(),
        }
    }
}

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D>(deserializer: D) -> Result<Chat, D::Error>
        where
            D: Deserializer<'de>,
    {
        let raw: RawChat = Deserialize::deserialize(deserializer)?;

        macro_rules! required_field {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        }

        Ok(match raw.type_.as_ref() {
            "private" => Chat::Private(User {
                id: raw.id.into(),
                username: raw.username,
                first_name: required_field!(first_name),
                last_name: raw.last_name,
                is_bot: false,
                language_code: raw.language_code,
            }),
            "group" => Chat::Group(Group {
                id: raw.id.into(),
                title: required_field!(title),
                all_members_are_administrators: required_field!(all_members_are_administrators),
                invite_link: raw.invite_link,
            }),
            "supergroup" => Chat::Supergroup(Supergroup {
                id: raw.id.into(),
                title: required_field!(title),
                username: raw.username,
                invite_link: raw.invite_link,
            }),
            "channel" => Chat::Channel(Channel {
                id: raw.id.into(),
                title: required_field!(title),
                username: raw.username,
                invite_link: raw.invite_link,
            }),
            _ => Chat::Unknown(raw),
        })
    }
}

/// This object represents a chat, directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct RawChat {
    /// Unique identifier for this chat.
    pub id: Integer,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: String,
    /// Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Invite link for this chat, specific to this bot.
    /// Does not apply to private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// IETF language tag of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// True if a group has ‘All Members Are Admins’ enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_members_are_administrators: Option<bool>,
}
