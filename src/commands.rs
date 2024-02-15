use crate::models::{State, Command};
use crate::telegram::basic_methods::{cancel, help, start, invalid_state};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, DpHandlerDescription};
use teloxide::dptree;
use teloxide::dptree::{case, Handler};
use teloxide::prelude::*;


pub fn schema() -> Handler<'static, DependencyMap, crate::errors::Result<()>, DpHandlerDescription>
{
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(start)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

        .branch(case![State::CreateCategorie].endpoint(send_categorie))
        .branch(case![State::AddTaskName { categorie }].endpoint(send_taskname))
        .branch(case![State::AddDescription { categorie, taskname }].endpoint(send_description))
        .branch(case![State::CreateTask { categorie, taskname, description }].endpoint(send_deadline))
        .branch(dptree::endpoint(invalid_state));


    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler).branch(callback_query_handler)
    //State::Start - initial state
}
