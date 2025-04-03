// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use clap::Args;
use nucs::Result;
use std::cmp::min;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

/// Delay for the specified time (with countdown)
#[derive(Args, Debug)]
pub struct Command {
    duration: f64,
}

const ONE_SECOND: Duration = Duration::from_secs(1);

pub fn app(args: &Command) -> Result<()> {
    let mut duration = Duration::from_secs_f64(args.duration);
    let mut wake_up = Instant::now();

    while !duration.is_zero() {
        let (mins, secs) = (duration.as_secs() / 60, duration.as_secs() % 60);
        print!("    {mins:02}:{secs:02}\r");
        std::io::stdout().flush().unwrap_or(());

        let delta = min(ONE_SECOND, duration);
        wake_up += delta;

        thread::sleep(wake_up - Instant::now());

        duration -= delta;
    }
    print!("         \r");

    Ok(())
}
