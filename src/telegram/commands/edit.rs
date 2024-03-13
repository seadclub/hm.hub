use crate::db::{
    insert_user, select_category, select_task, update_category, update_deadline,
    update_description, update_taskname,
};
use crate::models::{MyDialogue, State};
use crate::telegram::utils::{check_deadline, edit_buttons, get_telegram_user_id};
use teloxide::{
    prelude::*,
    requests::Requester,
    types::InlineKeyboardMarkup,
    types::{CallbackQuery, Message},
    Bot,
};

pub async fn edit(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
    if msg.text().unwrap() == "/edit" {
        bot.send_message(msg.chat.id, "Send the name of the task you want to edit")
            .await?;
        return Ok(());
    }

    let argument: Vec<&str> = msg.text().unwrap().split_whitespace().collect();
    if let Ok(_task) = select_task(argument[1]) {
        bot.send_message(msg.chat.id, "Edit a Task")
            .reply_markup(InlineKeyboardMarkup::new(edit_buttons()))
            .await?;
        dialogue
            .update(State::ReceiveEditChoice {
                taskname: argument[1].to_string(),
            })
            .await?;
    } else {
        bot.send_message(msg.chat.id, "Task not found").await?;
    }
    Ok(())
}

pub async fn receive_edit_button(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    taskname: String,
) -> crate::errors::Result<()> {
    insert_user(&get_telegram_user_id(&dialogue)).unwrap();
    if let Some(product) = &q.data {
        if product == "finish" {
            bot.send_message(
                dialogue.chat_id(),
                "Okey, I'm waiting for your next command",
            )
            .await?;
            dialogue.exit().await?;
        } else if product == "deadline" {
            bot.send_message(
                dialogue.chat_id(),
                "Send new deadline\nExample: 2023-04-12):",
            )
            .await?;
            dialogue.update(State::EditDeadline { taskname }).await?;
        } else if product == "description" {
            bot.send_message(dialogue.chat_id(), "Send new description:")
                .await?;
            dialogue.update(State::EditDescription { taskname }).await?;
        } else if product == "taskname" {
            bot.send_message(dialogue.chat_id(), "Send new name:")
                .await?;
            dialogue.update(State::EditTaskName { taskname }).await?;
        } else if product == "category" {
            bot.send_message(dialogue.chat_id(), "Send new category:")
                .await?;
            dialogue.update(State::EditCategory { taskname }).await?;
        }
    }

    Ok(())
}

pub async fn edit_taskname(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    taskname: String,
) -> crate::errors::Result<()> {
    if let Ok(_value) = update_taskname(&taskname, msg.text().unwrap()) {
        bot.send_message(
            dialogue.chat_id(),
            "Taskname has been updated successfully!",
        )
        .await?;

        bot.send_message(msg.chat.id, "Choose what you want to edit next")
            .reply_markup(InlineKeyboardMarkup::new(edit_buttons()))
            .await?;

        dialogue
            .update(State::ReceiveEditChoice {
                taskname: msg.text().unwrap().to_string(),
            })
            .await?;
    } else {
        bot.send_message(
            dialogue.chat_id(),
            "Task name already exists. Please enter a different name",
        )
        .await?;
    }

    Ok(())
}

pub async fn edit_description(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    taskname: String,
) -> crate::errors::Result<()> {
    update_description(&taskname, msg.text().unwrap()).unwrap();
    bot.send_message(
        dialogue.chat_id(),
        "Description has been updated successfully!",
    )
    .await?;

    bot.send_message(msg.chat.id, "Choose what you want to edit next")
        .reply_markup(InlineKeyboardMarkup::new(edit_buttons()))
        .await?;

    dialogue
        .update(State::ReceiveEditChoice { taskname })
        .await?;
    Ok(())
}

pub async fn edit_deadline(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    taskname: String,
) -> crate::errors::Result<()> {
    if check_deadline(&msg.text().unwrap()) {
        update_deadline(&taskname, msg.text().unwrap()).unwrap();
        bot.send_message(
            dialogue.chat_id(),
            "Description has been updated successfully!",
        )
        .await?;

        bot.send_message(msg.chat.id, "Choose what you want to edit next")
            .reply_markup(InlineKeyboardMarkup::new(edit_buttons()))
            .await?;

        dialogue
            .update(State::ReceiveEditChoice { taskname })
            .await?;
    } else {
        bot.send_message(
            dialogue.chat_id(),
            "Invalid deadline format. Please enter the deadline in the format: YYYY-MM-DD\nExample: 2023-04-12",
            )
            .await?;
    }
    Ok(())
}

pub async fn edit_category(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    taskname: String,
) -> crate::errors::Result<()> {
    if let Ok(category_id) = select_category(msg.text().unwrap()) {
        update_category(&taskname, &category_id.to_string()).unwrap();
        let message = format!("Category has been moved to {}", msg.text().unwrap());

        bot.send_message(dialogue.chat_id(), message).await?;

        bot.send_message(msg.chat.id, "Choose what you want to edit next")
            .reply_markup(InlineKeyboardMarkup::new(edit_buttons()))
            .await?;

        dialogue
            .update(State::ReceiveEditChoice { taskname })
            .await?;
    } else {
        bot.send_message(
            dialogue.chat_id(),
            "Category not found. Please enter a different category",
        )
        .await?;
    }
    Ok(())
}
