// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use clap::Args;
use nucs::{table, Result};
use std::{
    collections::VecDeque,
    io::{self, Write},
};

/// Show a table as a sparkline
#[derive(Args, Debug)]
pub struct Command {
    /// Watch the live results (animate a single line)
    #[arg(short, long)]
    watch: bool,

    /// Show history during live updates
    #[arg(short = 'H', long)]
    history: bool,
}

fn spark(val: f64, max: f64) -> char {
    if val < (0.125 * max) {
        '▁'
    } else if val < (0.25 * max) {
        '▂'
    } else if val < (0.375 * max) {
        '▃'
    } else if val < (0.5 * max) {
        '▄'
    } else if val < (0.625 * max) {
        '▅'
    } else if val < (0.75 * max) {
        '▆'
    } else if val < (0.875 * max) {
        '▇'
    } else {
        '█'
    }
}

fn sparkline<'a>(data: impl IntoIterator<Item = &'a f64>, max: f64) -> String {
    let mut s = String::new();

    for v in data {
        s.push(spark(*v, max));
    }

    s
}

pub fn app(args: &Command) -> Result<()> {
    let columns: usize = crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80);

    let (_headings, rows) = table::rows(io::stdin().lock())?;

    let mut history = VecDeque::new();

    for row in rows {
        if let Some(vals) = row.as_percent() {
            let sparks = sparkline(&vals, 100.0);

            if args.history {
                let mut avg = vals.iter().sum::<f64>() / vals.len() as f64;

                // this is numerically "wrong" but ensures that big single
                // values appear as a tiny spike in the history line
                if vals.iter().copied().max_by(f64::total_cmp).unwrap_or(0.0) > 66.67 {
                    avg = avg.max(0.125);
                }
                history.push_front(avg);

                // adjust history to fit within terminal width
                let len = columns.saturating_sub(12 + vals.len());
                if history.len() < len {
                    history.resize(len, 0.0);
                }
                while history.len() > len {
                    history.pop_back();
                }

                let history = sparkline(&history, 100.0);

                print!("    {sparks}    {history}\x1b[K\r");
                io::stdout().flush()?;
            } else if args.watch {
                print!("    {sparks}\x1b[K\r");
                io::stdout().flush()?;
            } else {
                println!("{sparks}");
            }
        }
    }
    if args.watch {
        println!("");
    }

    Ok(())
}
