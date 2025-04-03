// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use clap::Args;
use nucs::{proc, Result};
use std::thread;
use std::time::Duration;

/// CPU utilization summary
#[derive(Args, Debug)]
pub struct Command {
    /// Sampling interval
    #[arg(short, long)]
    interval: Option<f64>,
}

pub fn app(args: &Command) -> Result<()> {
    print!("TBL1\x08\x08\x08\x08");

    let interval = args.interval.map(|i| (i * 1000.0) as u64);
    let mut then = proc::Stat::new()?;

    for i in 0..then.cpus.len() {
        print!("|\x08 CPU{i:<3}");
    }
    println!("");

    let t = Duration::from_millis(if let Some(i) = interval { i / 5 } else { 1000 });
    thread::sleep(t);

    loop {
        let now = proc::Stat::new()?;

        for i in 0..then.cpus.len() {
            let u = now.cpus[i].utilization(&then.cpus[i]);
            print!(" {u:3.0}%  ");
        }
        println!("");

        if let Some(i) = interval {
            let t = Duration::from_millis(i);
            thread::sleep(t);
        } else {
            return Ok(());
        }

        then = now;
    }
}
