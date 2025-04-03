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

impl From<Multicall> for Cli {
    fn from(mc: Multicall) -> Self {
        Cli {
            command: mc.command,
        }
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[clap(multicall = true)]
struct Multicall {
    #[command(subcommand)]
    command: Commands,
}

mod cpu;
mod sleep;
mod sparks;

#[derive(Debug, Subcommand)]
enum Commands {
    Cpu(cpu::Command),
    #[clap(alias = "peels")]
    Sleep(sleep::Command),
    Sparks(sparks::Command),
}

fn main() {
    let cli = if let Ok(mc) = Multicall::try_parse() {
        mc.into()
    } else {
        Cli::parse()
    };

    let res = match cli.command {
        Commands::Cpu(args) => cpu::app(&args),
        Commands::Sleep(args) => sleep::app(&args),
        Commands::Sparks(args) => sparks::app(&args),
    };

    if let Err(e) = res {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
