// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::io::{self, BufWriter};
use std::path::PathBuf;

use anyhow::{Context, Result};

use kconfq::{Config, require_config};

pub fn print_config_path() -> Result<()> {
    println!("{}", require_config()?.path().to_string_lossy());
    Ok(())
}

pub fn print_config(path: &Option<PathBuf>) -> Result<()> {
    let config = match path {
        Some(path) => Config::new(path),
        None => require_config()?,
    };

    let mut reader = config
        .reader()
        .context("failed to get reader of the kernel config file")?;
    let mut stdout = BufWriter::new(io::stdout().lock());
    io::copy(&mut reader, &mut stdout)?;

    Ok(())
}

pub fn find_line(name: &str) -> Result<()> {
    let line = kconfq::find_line(name)?;
    println!("{line}");
    Ok(())
}

pub fn find_value(name: &str) -> Result<()> {
    let value = kconfq::find_value(name)?;
    println!("{value}");
    Ok(())
}
