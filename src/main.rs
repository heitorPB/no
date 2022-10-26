// no - like yes, but different
// Copyright (C) 2022 - Heitor de Bittencourt
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use signal_hook::consts::signal::*;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional string to output
    pattern: Vec<String>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let pattern = if cli.pattern.len() > 0 {
        cli.pattern.join(" ")
    } else {
        String::from("n")
    };

    let signal = Arc::new(AtomicBool::new(false));
    let signals = [SIGHUP, SIGINT, SIGTERM, SIGQUIT];
    for sig in signals {
        signal_hook::flag::register(sig, Arc::clone(&signal))?;
    }

    while !signal.load(Ordering::Relaxed) {
        println!("{}", pattern);
    }

    Ok(())
}
