use std::sync::LazyLock;

use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatAction, LinkPreviewOptions, Message, ParseMode, ReplyParameters, UserId},
    Bot, RequestError,
};

static BOT_ID: LazyLock<UserId> = LazyLock::new(|| {
    let value = std::env::var("BOT_ID").expect("BOT_ID must be defined");
    let id = value
        .parse::<u64>()
        .expect("BOT_ID must be a valid integer");
    UserId(id)
});

pub(crate) trait BotExt {
    async fn reply(&self, message: &Message, text: &str) -> Result<Message, RequestError>;
    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError>;
    async fn try_reply_silent(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError>;
    async fn replace_chat_message(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError>;
    async fn send_preview(
        &self,
        message: &Message,
        text: &str,
        get_preview_url: impl Fn(&Message) -> Option<String>,
    ) -> Result<Message, RequestError>;
    fn is_self_message(&self, message: &Message) -> bool;
}

impl BotExt for Bot {
    async fn reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        self.send_message(message.chat.id, text)
            .reply_parameters(ReplyParameters::new(message.id))
            .parse_mode(ParseMode::Html)
            .await
    }

    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        self.send_chat_action(message.chat.id, ChatAction::Typing)
            .await?;
        self.try_reply_silent(message, text).await
    }

    async fn try_reply_silent(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError> {
        if let Some(reply) = message.reply_to_message() {
            self.send_message(message.chat.id, text)
                .reply_parameters(ReplyParameters::new(reply.id))
                .parse_mode(ParseMode::Html)
                .await
        } else {
            self.send_message(message.chat.id, text)
                .parse_mode(ParseMode::Html)
                .await
        }
    }

    async fn replace_chat_message(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError> {
        let _del = self.delete_message(message.chat.id, message.id).await;
        self.try_reply_silent(message, text).await
    }

    async fn send_preview(
        &self,
        message: &Message,
        text: &str,
        get_preview_url: impl Fn(&Message) -> Option<String>,
    ) -> Result<Message, RequestError> {
        let preview_options = LinkPreviewOptions {
            is_disabled: false,
            url: get_preview_url(message),
            prefer_small_media: false,
            prefer_large_media: true,
            show_above_text: false,
        };
        let _del = self.delete_message(message.chat.id, message.id).await;
        if let Some(reply) = message.reply_to_message() {
            self.send_message(message.chat.id, text)
                .reply_parameters(ReplyParameters::new(reply.id))
                .link_preview_options(preview_options)
                .parse_mode(ParseMode::Html)
                .await
        } else {
            self.send_message(message.chat.id, text)
                .parse_mode(ParseMode::Html)
                .link_preview_options(preview_options)
                .await
        }
    }

    fn is_self_message(&self, message: &Message) -> bool {
        return if let Some(forwarder) = message.forward_from_user() {
            forwarder.id == *BOT_ID
        } else {
            message
                .from
                .as_ref()
                .map_or(false, |from| from.id.0 == BOT_ID.0)
        };
    }
}
