use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::json::prelude::*;
use crate::model::id::{RoleId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParseValue {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "users")]
    Users,
    #[serde(rename = "roles")]
    Roles,
}

/// A builder to manage the allowed mentions on a message,
/// used by the [`ChannelId::send_message`] and
/// [`ChannelId::edit_message`] methods.
///
/// # Examples
///
/// ```rust,ignore
/// use serenity::builder::ParseValue;
///
/// // Mention only the user 110372470472613888
/// m.allowed_mentions(|am| am.empty_parse().users(vec![110372470472613888]));
///
/// // Mention all users and the role 182894738100322304
/// m.allowed_mentions(|am| am.parse(ParseValue::Users).roles(vec![182894738100322304]));
///
/// // Mention all roles and nothing else
/// m.allowed_mentions(|am| am.parse(ParseValue::Roles));
///
/// // Mention all roles and users, but not everyone
/// m.allowed_mentions(|am| am.parse(ParseValue::Users).parse(ParseValue::Roles));
///
/// // Mention everyone and the users 182891574139682816, 110372470472613888
/// m.allowed_mentions(|am| {
///     am.parse(ParseValue::Everyone).users(vec![182891574139682816, 110372470472613888])
/// });
///
/// // Mention everyone and the message author.
/// m.allowed_mentions(|am| am.parse(ParseValue::Everyone).users(vec![msg.author.id]));
/// ```
///
/// [`ChannelId::send_message`]: crate::model::id::ChannelId::send_message
/// [`ChannelId::edit_message`]: crate::model::id::ChannelId::edit_message
#[derive(Clone, Debug)]
pub struct CreateAllowedMentions(pub HashMap<&'static str, Value>);

impl CreateAllowedMentions {
    /// Add a value that's allowed to be mentioned.
    ///
    /// If users or roles is specified, [`Self::users`] and [`Self::roles`] will not work.\
    /// If you use either, do not specify it's same type here.
    #[inline]
    pub fn parse(&mut self, value: ParseValue) -> &mut Self {
        let val = self.0.entry("parse").or_insert_with(|| Value::from(Vec::<Value>::new()));

        let arr = val.as_array_mut().expect("Must be an array");
        arr.push(json![value]);

        self
    }

    /// Clear all the values that would be mentioned.
    ///
    /// If parse is empty, the message will not mention anyone, unless they are specified on
    /// [`Self::users`] or [`Self::roles`].
    #[inline]
    pub fn empty_parse(&mut self) -> &mut Self {
        let val = self.0.entry("parse").or_insert_with(|| Value::from(Vec::<Value>::new()));

        let arr = val.as_array_mut().expect("Must be an array");
        arr.clear();

        self
    }

    /// Sets the users that will be allowed to be mentioned.
    #[inline]
    pub fn users<U: Into<UserId>>(&mut self, users: impl IntoIterator<Item = U>) -> &mut Self {
        self.0.insert(
            "users",
            Value::from({
                users.into_iter().map(|i| json!(i.into().to_string())).collect::<Vec<_>>()
            }),
        );
        self
    }

    /// Makes users unable to be mentioned.
    #[inline]
    pub fn empty_users(&mut self) -> &mut Self {
        let val = self.0.entry("users").or_insert_with(|| Value::from(Vec::<Value>::new()));

        let arr = val.as_array_mut().expect("Must be an array");
        arr.clear();

        self
    }

    /// Sets the roles that will be allowed to be mentioned.
    #[inline]
    pub fn roles<R: Into<RoleId>>(&mut self, roles: impl IntoIterator<Item = R>) -> &mut Self {
        self.0.insert(
            "roles",
            Value::from({
                roles.into_iter().map(|i| json!(i.into().to_string())).collect::<Vec<_>>()
            }),
        );
        self
    }

    /// Makes roles unable to be mentioned.
    #[inline]
    pub fn empty_roles(&mut self) -> &mut Self {
        let val = self.0.entry("roles").or_insert_with(|| Value::from(Vec::<Value>::new()));

        let arr = val.as_array_mut().expect("Must be an array");
        arr.clear();

        self
    }

    /// Makes the reply mention/ping the user.
    #[inline]
    pub fn replied_user(&mut self, mention_user: bool) -> &mut Self {
        self.0.insert("replied_user", Value::from(mention_user));

        self
    }
}

impl Default for CreateAllowedMentions {
    fn default() -> CreateAllowedMentions {
        let map = HashMap::new();
        CreateAllowedMentions(map)
    }
}
