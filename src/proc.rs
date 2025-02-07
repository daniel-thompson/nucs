// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use crate::{Error, Result};
use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::fs;
use std::ops::Sub;

#[derive(Add, AddAssign, Copy, Clone, Debug, Sub, SubAssign)]
pub struct Cpu {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
}

impl Cpu {
    fn new(s: &str) -> Result<Self> {
        let fields = s.split(' ').collect::<Vec<_>>();

        Ok(Cpu {
            user: fields.get(1).map_or(0, |f| f.parse().unwrap_or(0)),
            nice: fields.get(2).map_or(0, |f| f.parse().unwrap_or(0)),
            system: fields.get(3).map_or(0, |f| f.parse().unwrap_or(0)),
            idle: fields.get(4).map_or(0, |f| f.parse().unwrap_or(0)),
            iowait: fields.get(5).map_or(0, |f| f.parse().unwrap_or(0)),
            irq: fields.get(6).map_or(0, |f| f.parse().unwrap_or(0)),
            softirq: fields.get(7).map_or(0, |f| f.parse().unwrap_or(0)),
            steal: fields.get(8).map_or(0, |f| f.parse().unwrap_or(0)),
            guest: fields.get(9).map_or(0, |f| f.parse().unwrap_or(0)),
        })
    }

    fn sum(&self) -> u64 {
        self.user
            + self.nice
            + self.system
            + self.idle
            + self.iowait
            + self.irq
            + self.softirq
            + self.steal
            + self.guest
    }

    pub fn utilization(&self, other: &Self) -> f64 {
        let delta = self - other;
        (1.0 - (delta.idle as f64 / delta.sum() as f64)) * 100.0
    }
}

impl Sub for &Cpu {
    type Output = Cpu;

    fn sub(self, other: Self) -> Cpu {
        *self - *other
    }
}

pub struct Stat {
    pub cpu: Cpu,
    pub cpus: Vec<Cpu>,
}

impl Stat {
    pub fn new() -> Result<Self> {
        let stat = fs::read_to_string("/proc/stat")?;

        let mut it = stat.lines();
        let first_line = it
            .next()
            .ok_or(Error::MalformedData("/proc/stat".to_string()))?;
        let cpu = Cpu::new(first_line)?;

        let mut cpus = Vec::new();
        for ln in it {
            if ln.starts_with("cpu") {
                cpus.push(Cpu::new(ln)?);
            } else {
                break;
            }
        }

        Ok(Stat { cpu, cpus })
    }
}
