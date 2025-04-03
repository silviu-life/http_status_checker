use clap::Parser;
use clap_duration::duration_range_value_parse;
use duration_human::{DurationHuman, DurationHumanValidator};

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
    urls: Vec<String>
}

fn main() {
    let args = Args::parse();

    dbg!(&args);
}
