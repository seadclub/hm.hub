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

    // let callback_query_handler = Update::filter_message()
    //     .branch(case![State::GetEmail { phone_number }].endpoint(get_email))
    //     .branch(case![State::GetAge { phone_number }].endpoint(get_age))
    //     .branch(case![State::GetWeightAndHeight { phone_number }].endpoint(get_height_and_weight));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        // .branch(case![State::GetPhoneNumber].endpoint(get_number))
        // .branch(case![State::HomeTrainingMenu { phone_number }].endpoint(home_training_menu))
        // .branch(case![State::GymTrainingMenu { phone_number }].endpoint(gym_training_menu))
        .branch(dptree::endpoint(invalid_state));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
}