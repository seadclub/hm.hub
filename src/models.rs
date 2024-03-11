use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;
use teloxide::utils::command::BotCommands;

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "start the bot")]
    Start,
    #[command(description = "help message")]
    Help,
    #[command(description = "cancel current operation")]
    Cancel,
    #[command(description = "create new hometask")]
    Add,
    #[command(description = "edit hometask")]
    Edit,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceivAddChoice,
    CreateCategory,
    AddTaskName {
        category: String,
    },
    AddDescription {
        category: String,
        taskname: String,
    },
    CreateTask {
        category: String,
        taskname: String,
        description: String,
    },
    ReceivEditChoice {
        taskname: String,
    },
    EditTaskName {
        taskname: String,
    },
    EditDescription {
        taskname: String,
    },
    EditDeadline {
        taskname: String,
    },
    EditCategory {
        taskname: String,
    },
}

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub desc: String,
    pub deadline: String,
    pub date_created: String,
    pub category_id: i32,
}
