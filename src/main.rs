use crate::config::Config;
use clap::Parser;
use cli::Ayo;
use std::{env, thread, time::Duration};

mod cli;
mod config;
mod process;
mod util;

fn main() -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO")
    }

    env_logger::init();

    let Ayo {
        config_path,
        rules_path,
    } = Ayo::parse();

    log::debug!("Using config file: {config_path}");

    let mut config = Config::read_file(&config_path)?;
    config.merge_from_directory(&rules_path)?;

    log::info!(
        "Loaded {} group(s) and {} rule(s).",
        config.groups.len(),
        config.rules.len()
    );

    loop {
        process::process(&config)?;
        thread::sleep(Duration::from_secs(config.poll_interval));
    }
}
