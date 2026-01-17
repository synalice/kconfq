// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::io::{self, BufWriter};
use std::path::PathBuf;

use anyhow::{Context, Result};

use kconfq::require_config;

pub fn print_config_path(path: Option<PathBuf>) -> Result<()> {
    println!("{}", require_config(path)?.path().to_string_lossy());
    Ok(())
}

pub fn print_config(path: Option<PathBuf>) -> Result<()> {
    let config = require_config(path)?;
    let mut reader = config
        .reader()
        .context("failed to get reader of the kernel config file")?;
    let mut stdout = BufWriter::new(io::stdout().lock());

    io::copy(&mut reader, &mut stdout)?;

    Ok(())
}

pub fn find_line(name: &str, path: Option<PathBuf>) -> Result<()> {
    let config = require_config(path)?;
    let config_reader = config.reader()?;
    let line = kconfq::find_line(name, config_reader)?;

    println!("{line}");

    Ok(())
}

pub fn find_value(name: &str, path: Option<PathBuf>) -> Result<()> {
    let config = require_config(path)?;
    let config_reader = config.reader()?;
    let value = kconfq::find_value(name, config_reader)?;

    println!("{value}");

    Ok(())
}

pub fn is_gzip(path: Option<PathBuf>) -> Result<()> {
    if require_config(path)?.is_gzip()? {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}
