use std::{
    string::ToString,
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
    },
    time::Duration,
};

use anyhow::Result;
use log::*;
use teloxide::prelude::*;
use tokio::sync::broadcast;

mod bot;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config = Arc::new(config::read_config());
    info!("starting with config: {config:#?}");

    let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<()>(1);
    let shutdown = Arc::new(AtomicBool::new(false));
    let bot = bot::MyBot::new(config.clone()).await?;

    let sub_check_loop_handle = {
        let shutdown = shutdown.clone();
        tokio::task::spawn(async move {
            while !shutdown.load(Ordering::Acquire) {
                tokio::select! {
                   _ = tokio::time::sleep(Duration::from_secs(config.check_interval_secs)) => {}
                   _ = shutdown_rx.recv() => {
                       break
                   }
                }
            }
        })
    };
    let (bot_handle, bot_shutdown_token) = bot.spawn();

    {
        let shutdown_clone = shutdown.clone();
        ctrlc::set_handler(move || {
            info!("got signal Ctrl-C, shutting down...");
            shutdown_clone.swap(true, Ordering::Relaxed);
            let _res = bot_shutdown_token.shutdown();
            let _res = shutdown_tx.send(()).unwrap_or_else(|_| {
                // Makes the second Ctrl-C exit instantly
                std::process::exit(0);
            });
        })
        .expect("error setting Ctrl-C handler");
    }

    if let Err(err) = tokio::try_join!(bot_handle, sub_check_loop_handle) {
        panic!("{err}")
    }

    Ok(())
}
