// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use crate::{Error, Result};
use std::io::{BufRead, Lines};

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
        self.headings.0.push(title);
        for row in self.rows.iter_mut() {
            row.0.push(def.to_string());
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct Row(Vec<String>);

impl Row {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_percent(&self) -> bool {
        self.0.iter().map(|c| c.ends_with("%")).all(|b| b)
    }

    pub fn as_percent(&self) -> Option<Vec<f64>> {
        if !self.is_percent() {
            return None;
        }

        Some(
            self.0
                .iter()
                .map(|c| c.trim_end_matches("%").parse::<f64>().unwrap_or(0.0))
                .collect(),
        )
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
                row.0.push(field.trim().to_string());
                field = String::new();
                width = width_it.next().map(|w| w.clone());
            }
        }
        if field.len() != 0 {
            row.0.push(field.trim().to_string());
        }
        while row.0.len() < (self.widths.len() + 1) {
            row.0.push("".to_string());
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
            headings.0.push(field.trim().to_string());
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
        headings.0.push(field.trim().to_string());
    }

    Ok((headings, rows))
}
