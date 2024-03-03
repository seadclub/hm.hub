use crate::{db::select_all_categories, models::MyDialogue};
use chrono::NaiveDate;
use teloxide::types::InlineKeyboardButton;

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
            .map(|product| {
                vec![InlineKeyboardButton::callback(
                    product.to_string(),
                    product.to_string(),
                )]
            })
            .collect::<Vec<_>>();
        products.push(vec![additional_row[1].clone()]);
        return products;
    } else if page == 1 {
        let mut products = categories[..4]
            .iter()
            .map(|product| {
                vec![InlineKeyboardButton::callback(
                    product.to_string(),
                    product.to_string(),
                )]
            })
            .collect::<Vec<_>>();
        products.push(vec![additional_row[0].clone()]);
        return products;
    } else {
        let mut products = categories[((page - 1) * 4)..(page * 4)]
            .iter()
            .map(|product| {
                vec![InlineKeyboardButton::callback(
                    product.to_string(),
                    product.to_string(),
                )]
            })
            .collect::<Vec<_>>();
        products.push(additional_row);
        return products;
    }
}

pub fn get_telegram_user_id(msg: &MyDialogue) -> String {
    return msg.chat_id().to_string()
}

pub fn edit_buttons() -> Vec<Vec<InlineKeyboardButton>> {
    let edit_keys = vec!["Deadline", "Description", "TaskName", "Category"];
    let finish_button = InlineKeyboardButton::callback("Finish", "finish");

    let mut inlines_buttons = vec![];

    let edit_buttons = edit_keys
        .iter()
        .map(|button| {
            InlineKeyboardButton::callback(
                button.to_string(),
                button.to_string().to_lowercase(),
            )
        })
        .collect::<Vec<_>>();

    inlines_buttons.push(edit_buttons);
    inlines_buttons.push(vec![finish_button]);
    
    inlines_buttons
}