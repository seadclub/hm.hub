use teloxide::{requests::Requester, types::Message, Bot};

pub async fn start(bot: Bot, msg: Message) -> crate::errors::Result<()> {
    bot.send_message(msg.chat.id, "Welcome to HH!").await?;
    Ok(())
}
