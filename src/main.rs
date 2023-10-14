// This is a bot that asks you three questions, e.g. a simple test.
//
// # Example
// ```
//  - Hey
//  - Let's start! What's your full name?
//  - Gandalf the Grey
//  - How old are you?
//  - 223
//  - What's your location?
//  - Middle-earth
//  - Full name: Gandalf the Grey
//    Age: 223
//    Location: Middle-earth
// ```
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveOperation,
    CreatePhoto,
    ReceiveFullName,
    ReceiveAge {
        full_name: String,
    },
    ReceiveLocation {
        full_name: String,
        age: u8,
    },
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::CreatePhoto].endpoint(create_photo))
            .branch(dptree::case![State::ReceiveOperation].endpoint(receive_operation)),
        // .branch(dptree::case![State::ReceiveAge { full_name }].endpoint(receive_age))
        // .branch(
        // dptree::case![State::ReceiveLocation { full_name, age }].endpoint(receive_location),
        // ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Select an operation: Read, Create, Update, or Delete?",
    )
    .await?;
    dialogue.update(State::ReceiveOperation).await?;
    Ok(())
}

async fn receive_operation(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            match text.to_lowercase().as_str() {
                "read" => {
                    println!("Received READ command");
                    dialogue.update(State::CreatePhoto).await?;
                }
                "stop" => {
                    dialogue.exit().await?;
                }
                _ => {
                    println!("Received UNKNOWN command");
                }
            }
            // bot.send_message(msg.chat.id, "How old are you?").await?;
            // dialogue
            //     .update(State::ReceiveAge {
            //         full_name: text.into(),
            //     })
            //     .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

async fn create_photo(
    bot: Bot,
    dialogue: MyDialogue,
    // full_name: String, // Available from `State::ReceiveAge`.
    msg: Message,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "You entered the photo creation state!")
        .await?;
    // match msg.text() {
    //     Some(Ok(age)) => {
    //         bot.send_message(msg.chat.id, "What's your location?")
    //             .await?;
    //         dialogue
    //             .update(State::ReceiveLocation { full_name, age })
    //             .await?;
    //     }
    //     _ => {
    //         bot.send_message(msg.chat.id, "Send me a number.").await?;
    //     }
    // }

    Ok(())
}

// async fn receive_location(
//     bot: Bot,
//     dialogue: MyDialogue,
//     (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
//     msg: Message,
// ) -> HandlerResult {
//     match msg.text() {
//         Some(location) => {
//             let report = format!("Full name: {full_name}\nAge: {age}\nLocation: {location}");
//             bot.send_message(msg.chat.id, report).await?;
//             dialogue.exit().await?;
//         }
//         None => {
//             bot.send_message(msg.chat.id, "Send me plain text.").await?;
//         }
//     }
//
//     Ok(())
// }
