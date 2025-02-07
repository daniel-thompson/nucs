// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Daniel Thompson

use crate::{Error, Result};
use std::io::BufRead;

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
        let mut table = Table::new();
        let mut widths = Vec::<u32>::new();

        let mut it = f.lines();
        let first_line = it.next().ok_or(Error::EOF)??;

        if !first_line.starts_with("TBL1\x08\x08\x08\x08|\x08") {
            return Err(Error::MalformedData("<filename-missing>".to_string()));
        };

        // Parse the header line.
        let first_line = &first_line[10..];
        let mut field = String::new();
        let mut width = 0;
        for c in first_line.chars() {
            if c == '|' {
                table.add_column(field.trim().to_string(), "");
                field = String::new();
                widths.push(width);
                width = 1;
            } else if c == '\x08' {
                width -= 1;
            } else {
                field.push(c);
                width += 1;
            }
        }
        if field.len() != 0 {
            table.add_column(field.trim().to_string(), "");
        }

        // Parse each line
        for ln in it {
            let mut row = Row::new();
            let mut field = String::new();
            let mut width_it = widths.iter();
            let mut width = width_it.next().map(|w| w.clone());

            for c in ln?.chars() {
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
            while row.0.len() < table.headings.0.len() {
                row.0.push("".to_string());
            }

            table.rows.push(row);
        }

        Ok(table)
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
