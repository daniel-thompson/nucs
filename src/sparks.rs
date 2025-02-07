// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use clap::Args;
use nuts::{table::Table, Result};
use std::io;

/// Show a table as a sparkline
#[derive(Args, Debug)]
pub struct Command {}

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

pub fn app(_args: &Command) -> Result<()> {
    let stdin = io::stdin().lock();

    let tbl = Table::parse(stdin)?;

    for row in tbl.rows.iter() {
        if let Some(vals) = row.as_percent() {
            print!("{}", sparkline(&vals, 100.0));
        }
    }
    println!("");

    Ok(())
}
