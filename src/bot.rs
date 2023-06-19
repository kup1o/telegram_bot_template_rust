use std::sync::Arc;

use anyhow::Result;
use teloxide::{
    dispatching::DefaultKey,
    utils::command::BotCommands,
};

use crate::*;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "say hi")]
    Hi,
}

pub struct MyBot {
    pub dispatcher: Dispatcher<Arc<Bot>, anyhow::Error, DefaultKey>,
    pub tg: Arc<Bot>,
}

impl MyBot {
    pub async fn new(config: Arc<config::Config>) -> Result<Self> {
        let tg = Arc::new(Bot::new(config.telegram_bot_token.expose_secret()));
        tg.set_my_commands(Command::bot_commands()).await?;

        let handler = Update::filter_message().branch(
            dptree::filter(|msg: Message, config: Arc<config::Config>| {
                msg.from()
                    .map(|user| config.authorized_user_ids.contains(&user.id.0))
                    .unwrap_or_default()
            })
            .filter_command::<Command>()
            .endpoint(handle_command),
        );

        let dispatcher = Dispatcher::builder(tg.clone(), handler)
            .dependencies(dptree::deps![config.clone()])
            .default_handler(|upd| async move {
                warn!("unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "an error has occurred in the dispatcher",
            ))
            .build();

        let my_bot = MyBot {
            dispatcher,
            tg: tg.clone(),
        };
        Ok(my_bot)
    }

    pub fn spawn(
        mut self,
    ) -> (
        tokio::task::JoinHandle<()>,
        teloxide::dispatching::ShutdownToken,
    ) {
        let shutdown_token = self.dispatcher.shutdown_token();
        (
            tokio::spawn(async move { self.dispatcher.dispatch().await }),
            shutdown_token,
        )
    }
}

pub async fn handle_command(msg: Message, tg: Arc<Bot>, cmd: Command) -> Result<()> {
    async fn handle(msg: &Message, tg: &Bot, command: Command) -> Result<()> {
        match command {
            Command::Help => {
                tg.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Command::Hi => {
                let reply = "It works";
                tg.send_message(msg.chat.id, reply).await?;
            }
        };

        Ok(())
    }

    if let Err(err) = handle(&msg, &tg, cmd).await {
        error!("failed to handle message: {}", err);
        tg.send_message(msg.chat.id, "Something went wrong").await?;
    }

    Ok(())
}
