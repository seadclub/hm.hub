use crate::models::Command;
use crate::models::{MyDialogue, State};
use teloxide::prelude::*;
use teloxide::requests::Requester;
use teloxide::types::{ButtonRequest, KeyboardButton, KeyboardMarkup};
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

pub async fn help(bot: Bot, msg: Message) -> crate::errors::Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
    let button = KeyboardButton::new("Відправити номер").request(ButtonRequest::Contact);
    let markup = KeyboardMarkup::new([[button]])
        .resize_keyboard(true)
        .one_time_keyboard(true);
    bot.send_message(msg.chat.id, "Привіт! Я бот для тренувань! \n\n \
                Тут ти можешь зробити для себе тренування у залі, \
                тренування удома, та дієту, спеціально підібрану для тебе! \n\n \
                Будь-ласка, відправ спочатку свій номер телефона а потім пошту, для реєстрації або перевірки чи ти вже зареєстрованний!")
        .reply_markup(markup)
        .allow_sending_without_reply(true)
        .await?;

    dialogue.update(State::GetPhoneNumber).await?;
    Ok(())
}

pub async fn add(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
    let categories = select_all_categories().unwrap();
    let create_row =  InlineKeyboardButton::callback("create categorie", "create");
    let mut products = categories
        .iter()
        .map(|product| vec![InlineKeyboardButton::callback(product.to_string(), product.to_string())])
        .collect::<Vec<_>>();


    if categories.len() <= 4 {
        products.push(vec![create_row]);
        bot.send_message(msg.chat.id, "Select a product:")
            .reply_markup(InlineKeyboardMarkup::new(products))
            .await?;
        
        dialogue.update(State::ReceiveProductChoice).await?;
        return Ok(());
    } else {
        let mut products = categories[..4]
            .iter()
            .map(|product| vec![InlineKeyboardButton::callback(product.to_string(), product.to_string())])
            .collect::<Vec<_>>();

        let additional_row = ["next"]
            .iter()
            .map(|&product| InlineKeyboardButton::callback(product.to_string(), "next_2"))
            .collect();

        products.push(additional_row);
        products.push(vec![create_row]);

        bot.send_message(msg.chat.id, "Select a product:")
            .reply_markup(InlineKeyboardMarkup::new(products))
            .await?;
        
        dialogue.update(State::ReceiveProductChoice).await?;
    }

    // let additional_row = ["next", "previous"]
    //     .iter()
    //     .map(|&product| InlineKeyboardButton::callback(product.to_string(), product.to_string()))
    //     .collect();
    Ok(())
}

pub async fn receive_add_button(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
) -> crate::errors::Result<()> {
    insert_user(dialogue.clone()).unwrap();
    if let Some(product) = &q.data {
        if product == "create" {
            bot.send_message(
                dialogue.chat_id(),
                "Enter the name of the new product:",
            )
            .await?;
            dialogue.update(State::CreateCategorie).await?;
        } else if product.starts_with("next") || product.starts_with("previous") {
            let page = product.chars().last().unwrap().to_digit(10).unwrap() as usize;
            let create_row =  InlineKeyboardButton::callback("create categorie", "create");
            let mut products = pages(page);
            products.push(vec![create_row]);
            bot.send_message(
                dialogue.chat_id(),
                "Select a product:",
            )
            .reply_markup(InlineKeyboardMarkup::new(products))
            .await?;
        } else {
            bot.send_message(
                dialogue.chat_id(),
                "send task name:",
            )
            .await?;
            dialogue.update(State::AddTaskName { categorie: product.to_string() }).await?;
        }
    }

    Ok(())
}


pub async fn send_categorie(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
    insert_category(&msg.text().unwrap()).unwrap();
    bot.send_message(
        dialogue.chat_id(),
        "send task name:",
    )
    .await?;
    dialogue.update(State::AddTaskName { categorie: msg.text().unwrap().to_string() }).await?;

    Ok(())
}

pub async fn send_taskname(bot: Bot, dialogue: MyDialogue, msg: Message, categorie: String) -> crate::errors::Result<()> {
    bot.send_message(
        dialogue.chat_id(),
        "send description:",
    )
    .await?;
    dialogue.update(State::AddDescription { categorie: categorie, taskname: msg.text().unwrap().to_string() }).await?;
    Ok(())
}

pub async fn send_description(bot: Bot, dialogue: MyDialogue, msg: Message, (categorie, taskname) : (String, String)) -> crate::errors::Result<()> {
    bot.send_message(
        dialogue.chat_id(),
        "send deadline(example: 2023-04-12):",
    )
    .await?;
    dialogue.update(State::CreateTask { categorie: categorie, taskname: taskname, description: msg.text().unwrap().to_string() }).await?;
    Ok(())
}

pub async fn send_deadline(bot: Bot, dialogue: MyDialogue, msg: Message, (categorie, taskname, description) : (String, String, String)) -> crate::errors::Result<()> {
    if check_deadline(&msg.text().unwrap()) {
        insert_homework(&taskname, &description, &msg.text().unwrap(), &select_categorie(&categorie).unwrap(), dialogue.clone()).unwrap();
        bot.send_message(
            dialogue.chat_id(),
            "Task has been created successfully!",
        )
        .await?;
        dialogue.exit().await?;
    } else {
        bot.send_message(
            dialogue.chat_id(),
            "Invalid deadline format. Please enter the deadline in the format: YYYY-MM-DD",
        )
        .await?;
    }
    Ok(())
}

pub fn check_deadline(deadline: &str) -> bool {
    let date = NaiveDate::parse_from_str(deadline, "%Y-%m-%d");
    match date {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn pages(page: usize) -> Vec<Vec<InlineKeyboardButton>> {
    let categories = select_all_categories().unwrap();
    let additional_row: Vec<InlineKeyboardButton> = ["next", "previous"]
        .iter()
        .map(|&product| {
            let callback_data = match product {
                "next" => format!("next_{}", page + 1),
                "previous" => format!("previous_{}", page - 1),
                _ => panic!("Unknown product type"),
            };
            InlineKeyboardButton::callback(product.to_string(), callback_data)
        })
        .collect();

    if categories.len() <= page * 4 {
        let mut products = categories[((page - 1) * 4)..]
            .iter()
            .map(|product| vec![InlineKeyboardButton::callback(product.to_string(), product.to_string())])
            .collect::<Vec<_>>();
        products.push(vec![additional_row[1].clone()]);
        return products;
    } else if page == 1 {
        let mut products = categories[..4]
            .iter()
            .map(|product| vec![InlineKeyboardButton::callback(product.to_string(), product.to_string())])
            .collect::<Vec<_>>();
        products.push(vec![additional_row[0].clone()]);
        return products;
    } else {
        let mut products = categories[((page - 1) * 4)..(page * 4)]
            .iter()
            .map(|product| vec![InlineKeyboardButton::callback(product.to_string(), product.to_string())])
            .collect::<Vec<_>>();
        products.push(additional_row);
        return products;
    }
}
