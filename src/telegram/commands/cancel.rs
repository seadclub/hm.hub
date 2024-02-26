
use crate::models::MyDialogue;
use teloxide::{
    requests::Requester,
    types::Message,
    Bot,
};

pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
    bot.send_message(msg.chat.id, "Canceled").await?;
    dialogue.exit().await?;
    Ok(())
}