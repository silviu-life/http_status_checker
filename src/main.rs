use std::{
    fmt::Debug,
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use chrono::Utc;
use clap::{ArgAction, Parser, Subcommand};
use clap_duration::duration_range_value_parse;
use colored::Colorize;
use duration_human::{DurationHuman, DurationHumanValidator};
use futures::future::try_join_all;
use reqwest::StatusCode;
use tokio;

mod error;

/// HTTP status checker.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Verbose mode for http interactions.
    #[arg(short, long, action = ArgAction::Set, default_value_t = false)]
    verbose: bool,

    /// Timeout for each request.
    #[arg(
        short,
        long,
        default_value = "30s", 
        value_parser = duration_range_value_parse!(min: 1s, max: 10min)
    )]
    timeout: DurationHuman,

    /// Command to run.
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Get urls from command file.
    FromFile {
        /// File to get the urls from.
        file_path: PathBuf,
    },
    /// Get urls from command line.
    Urls {
        /// Urls to check the status on.
        #[arg(trailing_var_arg = true, num_args = 1..)]
        urls: Vec<String>,
    },
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

/// Check status of a url.
async fn check_status(
    url: String,
    timeout: Duration,
    verbose: bool,
) -> error::Result<PingResult> {
    let client = reqwest::Client::new();

    let now = Instant::now();
    if verbose {
        println!(
            "[{}] Starting request to {}.",
            Utc::now().to_rfc3339().color("red"),
            url.color("red")
        );
    }
    let result = match client.get(&url).timeout(timeout).send().await {
        Ok(result) => Ok(result),
        Err(e) => {
            if verbose {
                print!(
                    "[{}] Request to {} finished with error {}.",
                    Utc::now().to_rfc3339().color("red"),
                    url,
                    e
                );
            }
            Err(e)
        }
    }?;
    let elapsed = now.elapsed();
    let status = result.status();
    if verbose {
        println!(
            "[{}] Request to {} finished with status {}.",
            Utc::now().to_rfc3339().color("red"),
            url.color("red"),
            status.to_string().color("red")
        );
    }

    Ok(PingResult { url, elapsed, status })
}

/// Gets the urls specified in a command.
fn get_urls(command: SubCommand) -> error::Result<Vec<String>> {
    match command {
        SubCommand::FromFile { file_path } => {
            let contents = fs::read_to_string(file_path)?;
            Ok(contents.lines().map(|line| line.trim().to_string()).collect())
        }
        SubCommand::Urls { urls } => Ok(urls),
    }
}

#[tokio::main]
async fn main() -> error::Result<()> {
    let args = Args::parse();
    dbg!(&args);
    let timeout: Duration = (&args.timeout).into();
    let urls = get_urls(args.command)?;

    let futures = urls
        .into_iter()
        .map(|url| check_status(url, timeout, args.verbose))
        .collect::<Vec<_>>();

    let results = try_join_all(futures).await?;

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
