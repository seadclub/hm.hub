
use crate::models::{Command, State};
use crate::telegram::commands::*;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, DpHandlerDescription},
    dptree::{case, Handler},
    prelude::*,
    types::Update,
};

pub fn schema() -> Handler<'static, DependencyMap, crate::errors::Result<()>, DpHandlerDescription>
{
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Add].endpoint(add)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let callback_query_handler = Update::filter_callback_query().branch(
            case![State::ReceiveProductChoice].endpoint(receive_add_button),
        );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::CreateCategory].endpoint(send_category))
        .branch(case![State::AddTaskName { category }].endpoint(send_taskname))
        .branch(case![State::AddDescription { category, taskname }].endpoint(send_description))
        .branch(case![State::CreateTask { category, taskname, description }].endpoint(send_deadline))
        .branch(dptree::endpoint(invalid_state));


    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler).branch(callback_query_handler)
    //State::Start - initial state
}
