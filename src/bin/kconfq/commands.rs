// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use regex::Regex;

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

pub fn get_entry(name: &String) -> Result<()> {
    let config_reader = require_config()?
        .reader()
        .context("failed to get a reader to a kernel config file")?;

    let config_reader = BufReader::new(config_reader);
    let mut stdout = BufWriter::new(io::stdout().lock());

    let re = Regex::new(r"^(#\s+)?CONFIG_[A-Z_]+").expect("hardcoded regex should be valid");

    for line in config_reader.lines() {
        let line = line?;
        let line = line.trim();

        // Handle case where user inputs `name == "CONFIG_KCOV="`. This will
        // successfully find "CONFIG_KCOV=y", but it really should not.
        let line_before_equal_sign = line.split("=").collect::<Vec<&str>>();
        let line_before_equal_sign = match line_before_equal_sign.get(0) {
            Some(entry_name) => Ok(entry_name),
            None => Err(anyhow!("invalid input")),
        }?;

        if line_before_equal_sign.starts_with(name)
            || (line_before_equal_sign.starts_with(&format!("# {name}"))
                && line_before_equal_sign.ends_with(&format!("is not set")))
        {
            writeln!(stdout, "{line}")?;
            return Ok(());
        }
    }

    Err(anyhow!("entry {name} was not found"))
}
