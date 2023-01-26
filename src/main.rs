#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(let_chains)]
mod commands;
mod deamp;
#[cfg(feature = "ddinstagram")]
mod instagram;
mod logging;
mod message;
mod twitter;
mod utils;
mod youtube;

use crate::commands::Command;
use crate::logging::TeloxideLogger;
use dotenvy::dotenv;
use std::sync::{atomic::Ordering, Arc};
use teloxide::{
    dispatching::{update_listeners::Polling, HandlerExt, UpdateFilterExt},
    dptree,
    prelude::Dispatcher,
    types::{Message, Update},
    Bot,
};
use url::Host;

async fn run() {
    if let Err(e) = logging::init() {
        eprintln!("{e}");
        return;
    };
    dotenv().ok();

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(commands::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                let urls = utils::get_urls_from_message(&msg);
                let urls = utils::get_typed_urls(urls);
                let has_twitter_url = urls.iter().any(|url| {
                    url.host()
                        .map(|f| {
                            f == Host::Domain("twitter.com")
                                || f == Host::Domain("mobile.twitter.com")
                        })
                        .is_some()
                });
                twitter::FILTER_ENABLED.load(Ordering::Relaxed) && has_twitter_url
            })
            .endpoint(twitter::handler),
        );
    #[cfg(feature = "ddinstagram")]
    let handler = handler.branch(
        dptree::filter(|msg: Message| {
            let urls = utils::get_urls_from_message(&msg);
            let urls = utils::get_typed_urls(urls);
            let has_instagram_url = urls.iter().any(|url| {
                url.host()
                    .map(|f| f == Host::Domain("instagram.com"))
                    .is_some()
            });
            instagram::FILTER_ENABLED.load(Ordering::Relaxed) && has_instagram_url
        })
        .endpoint(instagram::handler),
    );
    let handler = handler.branch(
        dptree::filter(|msg: Message| {
            let urls = utils::get_urls_from_message(&msg);
            let urls = utils::get_typed_urls(urls);
            let has_youtube_url = urls.iter().any(|url| {
                url.host()
                    .map(|f| {
                        f == Host::Domain("youtube.com") || f == Host::Domain("www.youtube.com")
                    })
                    .is_some()
            });
            youtube::FILTER_ENABLED.load(Ordering::Relaxed) && has_youtube_url
        })
        .endpoint(youtube::handler),
    );

    let handler = handler.branch(dptree::filter(deamp::is_amp).endpoint(deamp::handler));

    let error_handler = Arc::new(TeloxideLogger::default());
    let listener = Polling::builder(bot.clone()).drop_pending_updates().build();
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, error_handler)
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}
