use std::sync::Arc;

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Extension,
    Router,
    Server,
};

// use axum::{Extension, Server};

use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};

use serde::Deserialize;
// use std::net::SocketAddr;

#[derive(Deserialize)]
struct PostData {
    message: String,
}

async fn forward(
    Query(data): Query<PostData>,
    Extension(bot): Extension<Arc<Bot>>,
) -> impl IntoResponse {
    // use teloxide::types::ChatId;

    let chat_id: i64 = -831239370; // 替换为你的群组 ID
    let chat_id = ChatId(chat_id);

    match bot.send_message(chat_id, &data.message).await {
        Ok(_) => (StatusCode::OK, "Message sent successfully".to_string()),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error sending message: {:?}", err),
        ),
    }
}

#[tokio::main]
async fn main() {
    let bot = Bot::new("are you token");

    let addr_server = ([0, 0, 0, 0], 9191).into();
    let addr_bot = ([0, 0, 0, 0], 8383).into();
    let state = Arc::new(bot.clone());

    let route = Router::new()
        .route("/forward", get(forward))
        .layer(Extension(state));

    // let app = Router::new().nest("/", forward_route);

    tokio::spawn(async move {
        Server::bind(&addr_server)
            .serve(route.into_make_service())
            .await
            .unwrap();
    });

    let url = "https://test.io".parse().unwrap();

    let webhook_listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr_bot, url))
        .await
        .expect("Couldn't setup webhook");

    Command::repl_with_listener(bot, answer, webhook_listener).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Use this command to save a URL")]
    Save(String),
    #[command(description = "handle user's chat ID")]
    ChatId,
    #[command(description = "bind lc project to a chat ID")]
    Bind,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
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

//-831239370
