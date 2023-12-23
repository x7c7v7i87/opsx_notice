use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use teloxide::{prelude::*, utils::command::BotCommands};

use crate::Data;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]

pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Use this command to save a URL")]
    Save(String),
    #[command(description = "handle user's chat ID")]
    ChatId,
    #[command(description = "bind lc project to a chat ID")]
    Bind,
}

pub async fn notice(
    // Query(data): Query<crate::Data>,
    Extension(bot): Extension<Arc<Bot>>,
    Json(from): Json<Data>,
) -> impl IntoResponse {
    let chat_id_result: Result<i64, _> = from.key.parse::<i64>();

    let chat_id = match chat_id_result {
        Ok(num) => num,
        Err(e) => {
            println!("to chat_id err: {}", e);
            return (StatusCode::BAD_REQUEST, "Invalid chat ID".to_string());
        }
    };

    // let chat_id: i64 = -1111111; // update is chat id
    let chat_id = ChatId(chat_id);
    let message = from.message.clone();

    match bot.send_message(chat_id, &message).await {
        Ok(_) => (StatusCode::OK, "Message sent successfully".to_string()),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error sending message: {:?}", err),
        ),
    }
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    match cmd {
        Command::Help => {
            println!("help");
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::ChatId => {
            bot.send_message(msg.chat.id, format!("Your chat ID is {chat_id}"))
                .await?
        }

        Command::Bind => {
            bot.send_message(msg.chat.id, format!("Your chat ID is {chat_id}"))
                .await?
        }

        Command::Save(text) => {
            bot.send_message(msg.chat.id, format!("The URL you want to save is: {text}"))
                .await?
        }
    };

    Ok(())
}
