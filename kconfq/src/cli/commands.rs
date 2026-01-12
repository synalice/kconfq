// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use anyhow::{Context, Result};

use flate2::read::GzDecoder;
use kconfq::require_config;

pub fn print_config_path() -> Result<()> {
    println!("{}", require_config()?.path().to_string_lossy());
    Ok(())
}

pub fn print_config() -> Result<()> {
    let config = require_config()?;
    let config_file = File::open(config.path()).context("failed to open config file")?;

    let is_gzip = config
        .is_gzip()
        .context("failed to check if config file is gzip-compressed")?;

    let mut stdout_writer = BufWriter::new(io::stdout().lock());

    if is_gzip {
        let mut reader = GzDecoder::new(config_file);
        io::copy(&mut reader, &mut stdout_writer)?;
    } else {
        let mut reader = BufReader::new(config_file);
        io::copy(&mut reader, &mut stdout_writer)?;
    }

    Ok(())
}
