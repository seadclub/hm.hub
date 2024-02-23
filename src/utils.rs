use crate::models::MyDialogue;

pub fn get_telegram_user_id(msg: &MyDialogue) -> String {
    return msg.chat_id().to_string()
}

pub fn check_deadline(deadline: &str) -> bool {
    let deadline = deadline.split('-').collect::<Vec<&str>>();
    if deadline.len() != 3 {
        return false;
    }
    if deadline[0].len() != 4 || deadline[1].len() != 2 || deadline[2].len() != 2 {
        return false;
    }
    return true;
}
