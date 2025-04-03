// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

mod cpu;
mod peels;
mod sparks;

#[derive(Debug, Subcommand)]
enum Commands {
    Cpu(cpu::Command),
    Peels(peels::Command),
    Sparks(sparks::Command),
}

fn main() {
    let res = match Cli::parse().command {
        Commands::Cpu(args) => cpu::app(&args),
        Commands::Peels(args) => peels::app(&args),
        Commands::Sparks(args) => sparks::app(&args),
    };

    if let Err(e) = res {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
