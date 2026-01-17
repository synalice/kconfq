// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Cli {
    /// Path to the config file
    #[arg(short, long, global = true)]
    path: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print path to the kernel config
    Path,
    /// Print the contents of the kernel config
    Config,
    /// Find config's line by its entry name
    Find {
        /// Example: CONFIG_CC_VERSION_TEXT
        entry_name: String,
    },
    /// Find value of the config's entry
    FindValue {
        /// Example: CONFIG_CC_VERSION_TEXT
        entry_name: String,
    },
    /// Print whenever the config's file is gzip-compressed or not
    IsGzip,
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    let path = cli.path;

    if let Some(command) = &cli.command {
        match command {
            Commands::Path => commands::print_config_path(path)?,
            Commands::Config => commands::print_config(path)?,
            Commands::Find { entry_name } => commands::find_line(entry_name, path)?,
            Commands::FindValue { entry_name } => commands::find_value(entry_name, path)?,
            Commands::IsGzip => commands::is_gzip(path)?,
        }
    }

    Ok(ExitCode::SUCCESS)
}
