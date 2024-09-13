mod steamerror;
mod telegram;

use std::{sync::{atomic::AtomicBool, Arc}, time::Duration};

use clap_derive::ValueEnum;
use reqwest;
use serde::Deserialize;
use clap::Parser;
use steamerror::ArgumentError;
use tokio_util::sync::CancellationToken;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(dead_code)]
#[derive(Deserialize)]
struct PriceOverview {
    currency: String,
    initial: u32,
    r#final: u32,
    discount_percent: u8,
    initial_formatted: String,
    final_formatted: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FeedChoice {
    Log,
    Telegram,
}

#[derive(Parser)]
#[command(name = "Gamemonitor")]
#[command(version = VERSION)]
#[command(about = "Monitors Steam games for discounts", long_about = None)]
struct Args {
    /// App ID on Steam. Usually it's the last 
    /// number contained in Store URL of the game.
    #[arg(short, long)]
    appid: String,
    /// Country code. Use this to get country-specific price.
    #[arg(long, default_value="")]
    cc: String,
    /// Minimal discount that will trigger the monitor.
    #[arg(short, long)]
    threshold: u8,
    /// How many seconds to wait between check. Should be no less than 2 secs.
    /// You may use `s', `m', `h', `d' suffixes to specify time in seconds, 
    /// minutes, hours or days. Default unit: seconds.
    #[arg(short, long)]
    delay: String,
    /// Way to notify about discounts.
    #[arg(value_enum, default_value_t=FeedChoice::Log)]
    feedtype: FeedChoice
}

#[tokio::main]
async fn main() -> steamerror::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let token = CancellationToken::new();
    let token_handle = token.clone();

    ctrlc::set_handler(move || {
        eprintln!("Exiting...");
        r.store(false, std::sync::atomic::Ordering::SeqCst);
        token_handle.cancel();
    }).expect("Error setting signal handler");

    let args: Args = Args::parse();

    let appid = &args.appid;
    let cc = &args.cc;
    let discount_bound = args.threshold;
    let delay = suffix_to_secs(&args.delay)?;

    let gamename = get_response_data::<String>(appid, "basic", cc, "name").await?;

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let token = token.clone();
        let overview: PriceOverview = serde_json::from_value(get_response_data(appid, "price_overview", cc, "price_overview").await?)
                                     .expect("Error parsing response: price_overview");

        if overview.discount_percent >= discount_bound {
            let msg = format!("DISCOUNT! {gamename} is for sale: {} now instead of {} ({}%)", 
                        overview.final_formatted, 
                        overview.initial_formatted,
                        overview.discount_percent
            );

            match args.feedtype {
                FeedChoice::Telegram => {
                    let tg = telegram::TelegramSender::new(
                               &std::env::var("TELEGRAM_API_TOKEN").expect("TELEGRAM_API_TOKEN is not set."), 
                                std::env::var("TELEGRAM_CHAT_ID")  .expect("TELEGRAM_CHAT_ID is not set.")
                                                           .parse().expect("Invalid chat id.")
                    );

                    dbg!(std::env::var("TELEGRAM_API_TOKEN"));
                    dbg!(std::env::var("TELEGRAM_CHAT_ID").unwrap().parse::<i64>());

                    tg.send_message(msg)?;
                },
                FeedChoice::Log => println!("{msg}"),
            };
        }

        let task = tokio::spawn(async move {
            tokio::select! {
                _ = token.cancelled() => {
                    dbg!("Cthulu cancels his great slumber...");
                }
                _ = tokio::time::sleep(delay) => {
                    dbg!("Cthulu still sleeps...");
                }
            }
        });

        let _ = tokio::join!(task);
    }

    Ok(())
}
 // {"2183900":{"success":true,"data":{"price_overview":{"currency":"RUB","initial":319900,"final":319900,"discount_percent":0,"initial_formatted":"","final_formatted":"3199 руб."}}}}

async fn get_response_data<T>(appid: &str, filterlist: &str, countrycode: &str, filter: &str) -> steamerror::SteamResult<T> 
where T: serde::de::DeserializeOwned
{
    // let filterlist = filterlist.join(",");
    let request_url = format!("https://store.steampowered.com/api/appdetails?appids={appid}&filters={filterlist}&cc={countrycode}");

    let resp = reqwest::get(&request_url).await?;

    let r = resp.error_for_status()?; 
    let serial = serde_json::from_slice::<serde_json::Value>(&r.bytes().await?)?;

    if serial[appid]["success"].as_bool().expect("Error parsing response") {
        Ok(serde_json::from_value(serial[appid]["data"][filter].clone())?) //.expect("Error parsing response: price_overview")
    } else {
        unimplemented!()
    }
}

fn suffix_to_secs(argtime: &str) -> Result<Duration, ArgumentError> {
    let unit = argtime.trim_start_matches(|ch: char| ch.is_ascii_digit());
    let value  = argtime.trim_end_matches(|ch: char| ch.is_ascii_alphabetic());

    let seconds = match unit {
        "s"|"" => value.parse::<u64>()?,
        "m"    => value.parse::<u64>()? * 60,
        "h"    => value.parse::<u64>()? * 3600,
        "d"    => value.parse::<u64>()? * 3600 * 24,
        _      => return Err(ArgumentError::UnitError),
    };

    Ok(Duration::from_secs(seconds))
}
