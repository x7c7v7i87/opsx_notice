// use anyhow::Result;

use std::sync::Arc;

use axum::{routing::post, Extension, Router, Server};

use teloxide::{dispatching::update_listeners::webhooks, prelude::*};

use opsx_notice::telegram::{answer, notice as telegram_notice, Command};

use opsx_notice::lark::msg::notice as lark_notice;

use opsx_notice::config::{BotCfg, LarkCfg};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cfg: BotCfg = BotCfg::new("./cfg.json").expect("Unable to parse JSON");

    let bot_token = cfg.telegram_bot_token.clone();
    let bot_uri = cfg.telegram_bot_uri.clone();
    //bot url for webhook
    let url = bot_uri.parse().unwrap();

    let lark_cfg: Arc<Vec<LarkCfg>> = Arc::new(cfg.lark_cfg);


    let bot = Bot::new(bot_token);

    let addr_web_server = ([0, 0, 0, 0], cfg.web_server_port).into();

    let addr_bot = ([0, 0, 0, 0], cfg.telegram_bot_port).into();

    let state: Arc<Bot> = Arc::new(bot.clone());

    let route = Router::new()
        .route("/notice/lark", post(lark_notice))
        .route("/notice/telegram", post(telegram_notice))
        .layer(Extension(state))
        .layer(Extension(lark_cfg));

    tokio::spawn(async move {
        Server::bind(&addr_web_server)
            .serve(route.into_make_service())
            .await
            .unwrap();
    });

    let webhook_listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr_bot, url))
        .await
        .expect("Couldn't setup webhook");

    Command::repl_with_listener(bot, answer, webhook_listener).await;
    // Ok(())
}
