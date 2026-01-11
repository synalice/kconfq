// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::{self, BufWriter};

use anyhow::{Context, Result};

use flate2::read::GzDecoder;
use kconfq::require_config_file;

pub fn print_config_path() -> Result<()> {
    println!("{}", require_config_file()?.to_string_lossy());
    Ok(())
}

pub fn print_config() -> Result<()> {
    let config_path = require_config_file()?;
    let config_file = File::open(config_path).context("failed to open config file")?;

    let mut decoder = GzDecoder::new(config_file);
    let mut out = BufWriter::new(io::stdout().lock());
    io::copy(&mut decoder, &mut out)?;

    Ok(())
}
