// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use cli::commands;

mod cli;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print path to the kernel config
    Path,
    /// Print the contents of the kernel config
    Config {
        /// Read kernel config from this path
        path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = &cli.command {
        match command {
            Commands::Path => commands::print_config_path()?,
            Commands::Config { path } => commands::print_config(path)?,
        }
    }

    Ok(())
}
