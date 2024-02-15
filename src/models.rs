
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;
use teloxide::utils::command::BotCommands;

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "cancel the purchase procedure.")]
    Cancel,
    #[command(description = "create new hometask")]
    Add,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveProductChoice,
    CreateCategorie,
    AddTaskName { categorie: String },
    AddDescription { categorie: String, taskname: String },
    CreateTask { categorie: String, taskname: String, description: String },
}
