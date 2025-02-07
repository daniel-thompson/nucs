// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

pub mod proc;
pub mod table;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected end of file")]
    EOF,

    #[error("I/O error")]
    IOError(#[from] std::io::Error),

    #[error("Malformed data in {0}")]
    MalformedData(String),

    #[error("Something bad happened")]
    SomethingBadHappened,
}

pub type Result<T> = std::result::Result<T, Error>;
