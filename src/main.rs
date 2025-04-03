use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use clap::Parser;
use clap_duration::duration_range_value_parse;
use colored::Colorize;
use duration_human::{DurationHuman, DurationHumanValidator};
use reqwest::StatusCode;
use tokio;

mod error;

/// HTTP status checker.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Verbose mode for http interactions.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Timeout for each request.
    #[arg(
        short,
        long,
        default_value = "30s", 
        value_parser = duration_range_value_parse!(min: 1s, max: 10min)
    )]
    timeout: DurationHuman,

    /// Urls to check the status on.
    #[arg(trailing_var_arg = true, num_args = 1..)]
    urls: Vec<String>,
}

/// The result of a ping to a website.
#[derive(Debug)]
struct PingResult {
    /// The pinged url.
    url: String,

    /// Ping duration.
    elapsed: Duration,

    /// Result status code.
    status: StatusCode,
}

async fn check_status(
    url: &str,
    timeout: &Duration,
) -> error::Result<PingResult> {
    let client = reqwest::Client::new();

    let now = Instant::now();
    let result = client.get(url).timeout(*timeout).send().await?;
    let elapsed = now.elapsed();
    let status = result.status();

    Ok(PingResult { url: url.to_string(), elapsed, status })
}

#[tokio::main]
async fn main() -> error::Result<()> {
    let args = Args::parse();

    dbg!(&args);

    let timeout: Duration = (&args.timeout).into();
    let mut results = vec![];
    for url in args.urls.iter() {
        let status = check_status(url, &timeout).await?;
        results.push(status);
    }

    for result in results {
        let duration_human = DurationHuman::from(result.elapsed);
        println!(
            "{}, status: {}, timeout: {}",
            result.url.color("blue"),
            result.status.to_string().color("red"),
            format!("{:#}", duration_human).color("purple")
        );
    }

    Ok(())
}
