// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use clap::Args;
use nuts::{table, Result};
use std::io::{self, Write};

/// Show a table as a sparkline
#[derive(Args, Debug)]
pub struct Command {
    /// Watch the live results (animate a single line)
    #[arg(short, long)]
    watch: bool,
}

fn sparkline(data: &[f64], max: f64) -> String {
    let mut s = String::new();
    for v in data.iter() {
        s.push(if *v < (0.125 * max) {
            '▁'
        } else if *v < (0.25 * max) {
            '▂'
        } else if *v < (0.375 * max) {
            '▃'
        } else if *v < (0.5 * max) {
            '▄'
        } else if *v < (0.625 * max) {
            '▅'
        } else if *v < (0.75 * max) {
            '▆'
        } else if *v < (0.875 * max) {
            '▇'
        } else {
            '█'
        });
    }
    s
}

pub fn app(args: &Command) -> Result<()> {
    let (_headings, rows) = table::rows(io::stdin().lock())?;
    for row in rows {
        if let Some(vals) = row.as_percent() {
            if args.watch {
                print!("    {}\r", sparkline(&vals, 100.0));
                io::stdout().flush()?;
            } else {
                println!("{}", sparkline(&vals, 100.0));
            }
        }
    }
    if args.watch {
        println!("");
    }

    Ok(())
}
