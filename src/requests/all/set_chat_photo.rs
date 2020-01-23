use serde::Serialize;

use super::BotWrapper;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, InputFile, True},
    Bot,
};

/// Use this method to set a new profile photo for the chat.
///
/// Photos can't be changed for private chats. The bot must be an administrator
/// in the chat for this to work and must have the appropriate admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#setchatphoto).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct SetChatPhoto<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
    photo: InputFile,
}

#[async_trait::async_trait]
impl Request for SetChatPhoto<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatPhoto",
            &self,
        )
        .await
    }
}

impl<'a> SetChatPhoto<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, photo: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot: BotWrapper(bot),
            chat_id,
            photo,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// New chat photo, uploaded using multipart/form-data.
    pub fn photo(mut self, val: InputFile) -> Self {
        self.photo = val;
        self
    }
}
