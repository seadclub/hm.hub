use crate::db::{
    insert_category, insert_homework, insert_user, select_all_categories, select_category, select_task,
};
use crate::models::{MyDialogue, State};
use crate::telegram::utils::{check_deadline, get_telegram_user_id, pages};
use teloxide::{
    prelude::*,
    requests::Requester,
    types::{CallbackQuery, Message},
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    Bot,
};

pub async fn add(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
    let categories = select_all_categories().unwrap();
    let create_row = InlineKeyboardButton::callback("Add subject", "create");
    let mut products = categories
        .iter()
        .map(|product| {
            vec![InlineKeyboardButton::callback(
                product.to_string(),
                product.to_string(),
            )]
        })
        .collect::<Vec<_>>();

    if categories.len() <= 4 {
        products.push(vec![create_row]);
        bot.send_message(msg.chat.id, "Select a subject:")
            .reply_markup(InlineKeyboardMarkup::new(products))
            .await?;

        dialogue.update(State::ReceivAddChoice).await?;
        return Ok(());
    } else {
        let mut products = categories[..4]
            .iter()
            .map(|product| {
                vec![InlineKeyboardButton::callback(
                    product.to_string(),
                    product.to_string(),
                )]
            })
            .collect::<Vec<_>>();

        let additional_row = ["next"]
            .iter()
            .map(|&product| InlineKeyboardButton::callback(product.to_string(), "next_2"))
            .collect();

        products.push(additional_row);
        products.push(vec![create_row]);

        bot.send_message(msg.chat.id, "Select a subject:")
            .reply_markup(InlineKeyboardMarkup::new(products))
            .await?;

        dialogue.update(State::ReceivAddChoice).await?;
    }

    Ok(())
}

pub async fn receive_add_button(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
) -> crate::errors::Result<()> {
    insert_user(&get_telegram_user_id(&dialogue)).unwrap();
    if let Some(product) = &q.data {
        if product == "create" {
            bot.send_message(dialogue.chat_id(), "Enter the name of the new subject:")
                .await?;
            dialogue.update(State::CreateCategory).await?;
        } else if product.starts_with("next") || product.starts_with("previous") {
            let page = product.chars().last().unwrap().to_digit(10).unwrap() as usize;
            let create_row = InlineKeyboardButton::callback("Create a new subject", "create");
            let mut products = pages(page);
            products.push(vec![create_row]);
            bot.send_message(dialogue.chat_id(), "Select a subject:")
                .reply_markup(InlineKeyboardMarkup::new(products))
                .await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Subject name:")
                .await?;
            dialogue
                .update(State::AddTaskName {
                    category: product.to_string(),
                })
                .await?;
        }
    }

    Ok(())
}

pub async fn send_category(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> crate::errors::Result<()> {
    insert_category(&msg.text().unwrap()).unwrap();
    bot.send_message(dialogue.chat_id(), "Send homework name:")
        .await?;
    dialogue
        .update(State::AddTaskName {
            category: msg.text().unwrap().to_string(),
        })
        .await?;

    Ok(())
}

pub async fn send_taskname(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    category: String,
) -> crate::errors::Result<()> {
    if let Ok(_task) = select_task(msg.text().unwrap()) {
        bot.send_message(dialogue.chat_id(), "Task already exists, Send another name")
            .await?;
    } else {
        bot.send_message(dialogue.chat_id(), "Send description of the homework:")
            .await?;
        dialogue
            .update(State::AddDescription {
                category,
                taskname: msg.text().unwrap().to_string(),
            })
            .await?;
    }
    
    Ok(())
}

pub async fn send_description(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    (category, taskname): (String, String),
) -> crate::errors::Result<()> {
    bot.send_message(dialogue.chat_id(), "Send deadline\nExample: 2023-04-12):")
        .await?;
    dialogue
        .update(State::CreateTask {
            category,
            taskname,
            description: msg.text().unwrap().to_string(),
        })
        .await?;
    Ok(())
}

pub async fn send_deadline(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    (category, taskname, description): (String, String, String),
) -> crate::errors::Result<()> {
    if check_deadline(&msg.text().unwrap()) {
        insert_homework(
            &taskname,
            &description,
            &msg.text().unwrap(),
            &select_category(&category).unwrap(),
            &get_telegram_user_id(&dialogue),
        )
        .unwrap();
        bot.send_message(dialogue.chat_id(), "Task has been created successfully!")
            .await?;
        dialogue.exit().await?;
    } else {
        bot.send_message(
            dialogue.chat_id(),
            "Invalid deadline format. Please enter the deadline in the format: YYYY-MM-DD\nExample: 2023-04-12",
        )
        .await?;
    }
    Ok(())
}
