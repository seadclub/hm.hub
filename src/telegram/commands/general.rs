use teloxide::{requests::Requester, types::Message, Bot};

pub async fn invalid_state(bot: Bot, msg: Message) -> crate::errors::Result<()> {
    bot.send_message(
        msg.chat.id,
        "I don't understand you. Use /help for the list of available commands.",
    )
    .await?;
    Ok(())
}
