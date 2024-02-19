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
    #[command(description = "display this text")]
    Help,
    #[command(description = "cancel the purchase procedure")]
    Cancel,
    #[command(description = "create new hometask")]
    Add,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveProductChoice,
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
}
