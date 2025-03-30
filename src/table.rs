// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use crate::{Error, Result};
use std::{
    fmt::{self, Display},
    io::{BufRead, Lines},
};

#[derive(Clone, Debug)]
pub struct Table {
    pub headings: Row,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            headings: Row::new(),
            rows: Vec::new(),
        }
    }

    pub fn parse(f: impl BufRead) -> Result<Self> {
        let (headings, iter) = rows(f)?;
        let rows = iter.collect();
        Ok(Table { headings, rows })
    }

    pub fn add_column(&mut self, title: String, def: &str) -> &mut Self {
        self.headings.0.push(title.into());
        for row in self.rows.iter_mut() {
            row.0.push(def.into());
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct Row(Vec<Cell>);

impl Row {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_percent(&self) -> bool {
        self.0.iter().all(|c| matches!(c, Cell::Percent(_)))
    }

    pub fn as_percent(&self) -> Option<Vec<f64>> {
        self.0
            .iter()
            .map(|c| {
                if let Cell::Percent(p) = c {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct Rows<T: BufRead> {
    iter: Lines<T>,
    widths: Vec<u32>,
}

impl<T: BufRead> Iterator for Rows<T> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let ln = self.iter.next()?.ok()?;

        let mut row = Row::new();
        let mut field = String::new();
        let mut width_it = self.widths.iter();
        let mut width = width_it.next().map(|w| w.clone());

        for c in ln.chars() {
            if c == '\x08' {
                width = width.map(|w| w + 1);
            } else {
                field.push(c);
                width = width.map(|w| w - 1);
            }

            if width == Some(0) {
                row.0.push(field.trim().into());
                field = String::new();
                width = width_it.next().map(|w| w.clone());
            }
        }
        if field.len() != 0 {
            row.0.push(field.trim().into());
        }
        while row.0.len() < (self.widths.len() + 1) {
            row.0.push("".into());
        }

        Some(row)
    }
}

pub fn rows<T: BufRead>(f: T) -> Result<(Row, Rows<T>)> {
    let mut headings = Row::new();
    let mut rows = Rows {
        iter: f.lines(),
        widths: Vec::new(),
    };

    let first_line = rows.iter.next().ok_or(Error::EOF)??;
    if !first_line.starts_with("TBL1\x08\x08\x08\x08|\x08") {
        return Err(Error::MalformedData("<filename-missing>".to_string()));
    };

    // Parse the header line.
    let first_line = &first_line[10..];
    let mut field = String::new();
    let mut width = 0;
    for c in first_line.chars() {
        if c == '|' {
            headings.0.push(field.trim().into());
            field = String::new();
            rows.widths.push(width);
            width = 1;
        } else if c == '\x08' {
            width -= 1;
        } else {
            field.push(c);
            width += 1;
        }
    }
    if field.len() != 0 {
        headings.0.push(field.trim().into());
    }

    Ok((headings, rows))
}

#[derive(Clone, Debug)]
pub enum Cell {
    Empty,
    Float(f64),
    Integer(i64),
    Percent(f64),
    String(String),
}

impl Cell {
    pub fn new() -> Self {
        Self::Empty
    }

    pub fn ends_with(&self, needle: &str) -> bool {
        if let Self::String(s) = self {
            s.ends_with(needle)
        } else {
            false
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Float(v) => write!(f, "{v}"),
            Self::Integer(v) => write!(f, "{v}"),
            Self::Percent(v) => write!(f, "{v}%"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

fn parse_cell_from_str(value: &str) -> Option<Cell> {
    if value == "" {
        Some(Cell::Empty)
    } else if value.ends_with("%")
        && value
            .chars()
            .rev()
            .skip(1)
            .all(|c| c.is_ascii_digit() || c == '.')
    {
        Some(Cell::Percent(
            value.trim_end_matches("%").parse().unwrap_or(0.0),
        ))
    } else {
        None
    }
}

impl From<String> for Cell {
    fn from(value: String) -> Self {
        if let Some(cell) = parse_cell_from_str(&value) {
            cell
        } else {
            Cell::String(value)
        }
    }
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        if let Some(c) = parse_cell_from_str(value) {
            c
        } else {
            Self::String(value.to_string())
        }
    }
}
