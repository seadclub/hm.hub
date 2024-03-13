use crate::models::Command;
use teloxide::{requests::Requester, types::Message, utils::command::BotCommands, Bot};

pub async fn help(bot: Bot, msg: Message) -> crate::errors::Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
