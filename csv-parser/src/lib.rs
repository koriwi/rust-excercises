use finite_state_machine::state_machine;
use std::{error::Error, fmt};

pub type CSVRow<'a> = Vec<Option<&'a str>>;
#[derive(Debug, PartialEq)]
pub struct CSVData<'a> {
    column_names: Vec<&'a str>,
    rows: Vec<CSVRow<'a>>,
}
impl<'a> CSVData<'a> {
    fn new(column_names: Vec<&'a str>, rows: Vec<CSVRow<'a>>) -> Self {
        CSVData { column_names, rows }
    }
    fn push_column(&mut self, column_name: &'a str) -> Result<(), &'static str> {
        self.column_names.push(column_name);
        Ok(())
    }
    fn push_value(&mut self, value: Option<&'a str>) -> Result<(), &'static str> {
        self.rows
            .last_mut()
            .ok_or("rows cannot be empty, impossible")?
            .push(value);
        Ok(())
    }
    fn add_empty_row(&mut self) -> Result<(), &'static str> {
        if let Some(row) = self.rows.last() {
            if row.len() != self.column_names.len() {
                return Err("row length does not match column length");
            }
        }
        self.rows.push(Vec::with_capacity(self.column_names.len()));
        Ok(())
    }
}

#[derive(Default)]
pub struct Data<'a> {
    trim_quotes: bool,
    input: Option<&'a str>,
    index: usize,
    delimiter: u8,
    parsed_csv: Option<CSVData<'a>>,
}
impl<'a> Data<'a> {
    fn store_cs_value(&mut self, is_header: bool) -> Result<(), &'static str> {
        let value = &self.input.ok_or("input is empty")?[..self.index];

        let parsed_csv = self
            .parsed_csv
            .as_mut()
            .ok_or("parsed_csv is undefined, impossible")?;
        // this smells
        if is_header {
            if value.is_empty() {
                return Err("value cannot be empty in header")?;
            }
            parsed_csv.push_column(value)?;
        } else if value.is_empty() {
            parsed_csv.push_value(None)?;
        } else {
            parsed_csv.push_value(Some(value))?;
        };
        self.skip_char_and_set_start()?;
        Ok(())
    }
    fn add_empty_row(&mut self) -> Result<(), &'static str> {
        self.parsed_csv
            .as_mut()
            .ok_or("parsed_csv is undefined, impossible")?
            .add_empty_row()?;
        Ok(())
    }
    fn store_char(&mut self) -> Result<(), &'static str> {
        self.index += 1;
        Ok(())
    }
    #[inline(always)]
    fn skip_char_and_set_start(&mut self) -> Result<(), &'static str> {
        self.input = Some(&self.input.ok_or("input is empty")?[self.index + 1..]);
        self.index = 0;
        Ok(())
    }
}
impl<'a> fmt::Debug for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buffer = match self.input {
            Some(text) => text.get(..self.index),
            None => None,
        };
        f.debug_struct("raw self")
            .field("index", &self.index)
            .field("found", &self.parsed_csv)
            .field("buffer", &buffer)
            // .field("remaining text", &self.input)
            .finish()
    }
}
state_machine!(
    CsvParser<'a>(Data<'a>);
    Start {
        Begin => FindHeaderDelimiter
    },
    FindHeaderDelimiter {
        FoundDelimiter => FindHeaderDelimiter,
        FoundLeftQuote => FindHeaderRightQuote,
        FoundNewLine => FindBodyDelimiter,
        FoundElse => FindHeaderDelimiter,
        Empty => End
    },
    FindHeaderRightQuote {
        FoundRightQuote => IgnoreNextHeaderDelimiter,
        FoundElse => FindHeaderRightQuote
    },
    FindBodyDelimiter {
        FoundDelimiter => FindBodyDelimiter,
        FoundLeftQuote => FindBodyRightQuote,
        FoundNewLine => FindBodyDelimiter,
        FoundElse => FindBodyDelimiter,
        Empty => End
    },
    FindBodyRightQuote {
        FoundRightQuote => IgnoreNextBodyDelimiter,
        FoundElse => FindBodyRightQuote
    },
    IgnoreNextHeaderDelimiter {
        FoundDelimiter => FindHeaderDelimiter,
        FoundNewLine => FindBodyDelimiter
    },
    IgnoreNextBodyDelimiter {
        FoundDelimiter => FindBodyDelimiter,
        FoundNewLine => FindBodyDelimiter
    }
);

pub use csv_parser::*;

impl<'a> StartTransitions for CsvParser<'a> {
    fn illegal(&mut self) {}
    fn begin(&mut self) -> Result<(), &'static str> {
        let lines = self
            .data
            .input
            .ok_or("input cannot be empty")?
            .chars()
            .take_while(|ch| ch.is_whitespace() && *ch != '\n')
            .count();
        self.data.parsed_csv = Some(CSVData::new(vec![], Vec::with_capacity(lines)));
        Ok(())
    }
}

impl<'a> FindHeaderDelimiterTransitions for CsvParser<'a> {
    fn illegal(&mut self) {}
    fn found_else(&mut self) -> Result<(), &'static str> {
        self.data.store_char()?;
        Ok(())
    }
    fn found_delimiter(&mut self) -> Result<(), &'static str> {
        self.data.store_cs_value(true)?;
        Ok(())
    }
    fn empty(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
    fn found_left_quote(&mut self) -> Result<(), &'static str> {
        self.data.skip_char_and_set_start()?;
        Ok(())
    }
    fn found_new_line(&mut self) -> Result<(), &'static str> {
        self.data.store_cs_value(true)?;
        self.data.add_empty_row()?;
        Ok(())
    }
}

impl<'a> FindHeaderRightQuoteTransitions for CsvParser<'a> {
    fn illegal(&mut self) {}

    fn found_else(&mut self) -> Result<(), &'static str> {
        FindHeaderDelimiterTransitions::found_else(self)
    }
    fn found_right_quote(&mut self) -> Result<(), &'static str> {
        self.data.store_cs_value(true)?;
        Ok(())
    }
}

impl<'a> IgnoreNextHeaderDelimiterTransitions for CsvParser<'a> {
    fn illegal(&mut self) {}
    fn found_delimiter(&mut self) -> Result<(), &'static str> {
        self.data.skip_char_and_set_start()
    }
    fn found_new_line(&mut self) -> Result<(), &'static str> {
        self.data.add_empty_row()?;
        self.data.skip_char_and_set_start()
    }
}

impl<'a> IgnoreNextBodyDelimiterTransitions for CsvParser<'a> {
    fn illegal(&mut self) {}
    fn found_delimiter(&mut self) -> Result<(), &'static str> {
        self.data.skip_char_and_set_start()
    }
    fn found_new_line(&mut self) -> Result<(), &'static str> {
        self.data.add_empty_row()?;
        self.data.skip_char_and_set_start()
    }
}

impl<'a> FindBodyDelimiterTransitions for CsvParser<'a> {
    fn illegal(&mut self) {}
    fn found_new_line(&mut self) -> Result<(), &'static str> {
        self.data.store_cs_value(false)?;
        self.data.add_empty_row()?;
        Ok(())
    }
    fn found_else(&mut self) -> Result<(), &'static str> {
        self.data.store_char()?;
        Ok(())
    }
    fn found_delimiter(&mut self) -> Result<(), &'static str> {
        self.data.store_cs_value(false)?;
        Ok(())
    }
    fn empty(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
    fn found_left_quote(&mut self) -> Result<(), &'static str> {
        self.data.skip_char_and_set_start()?;
        Ok(())
    }
}

impl<'a> FindBodyRightQuoteTransitions for CsvParser<'a> {
    fn illegal(&mut self) {}
    fn found_right_quote(&mut self) -> Result<(), &'static str> {
        self.data.store_cs_value(false)?;
        Ok(())
    }
    fn found_else(&mut self) -> Result<(), &'static str> {
        self.data.store_char()?;
        Ok(())
    }
}

impl<'a> Deciders for CsvParser<'a> {
    fn start(&self) -> StartEvents {
        StartEvents::Begin
    }
    fn find_header_delimiter(&self) -> FindHeaderDelimiterEvents {
        let char = match self.data.input {
            Some(input) => input.as_bytes().get(self.data.index),
            None => return FindHeaderDelimiterEvents::Empty,
        };
        match char {
            Some(c) if c == &self.data.delimiter => FindHeaderDelimiterEvents::FoundDelimiter,
            Some(c) if self.data.trim_quotes && *c == b'"' => {
                FindHeaderDelimiterEvents::FoundLeftQuote
            }
            Some(c) if *c == b'\n' => FindHeaderDelimiterEvents::FoundNewLine,
            _ => FindHeaderDelimiterEvents::FoundElse,
        }
    }
    fn find_header_right_quote(&self) -> FindHeaderRightQuoteEvents {
        let char = match self.data.input {
            Some(input) if !input.is_empty() => input.as_bytes().get(self.data.index),
            _ => return FindHeaderRightQuoteEvents::Illegal,
        };
        match char {
            Some(c) if c == &b'"' => FindHeaderRightQuoteEvents::FoundRightQuote,
            _ => FindHeaderRightQuoteEvents::FoundElse,
        }
    }
    fn find_body_delimiter(&self) -> FindBodyDelimiterEvents {
        let char = match self.data.input {
            Some(input) if !input.is_empty() => input.as_bytes().get(self.data.index),
            _ => return FindBodyDelimiterEvents::Empty,
        };
        match char {
            Some(c) if c == &self.data.delimiter => FindBodyDelimiterEvents::FoundDelimiter,
            Some(c) if self.data.trim_quotes && *c == b'"' => {
                FindBodyDelimiterEvents::FoundLeftQuote
            }
            Some(c) if *c == b'\n' => FindBodyDelimiterEvents::FoundNewLine,
            _ => FindBodyDelimiterEvents::FoundElse,
        }
    }
    fn find_body_right_quote(&self) -> FindBodyRightQuoteEvents {
        let char = match self.data.input {
            Some(input) if !input.is_empty() => input.as_bytes().get(self.data.index),
            _ => return FindBodyRightQuoteEvents::Illegal,
        };
        match char {
            Some(c) if c == &b'"' => FindBodyRightQuoteEvents::FoundRightQuote,
            _ => FindBodyRightQuoteEvents::FoundElse,
        }
    }
    fn ignore_next_header_delimiter(&self) -> IgnoreNextHeaderDelimiterEvents {
        let char = match self.data.input {
            Some(input) if !input.is_empty() => input.as_bytes().get(self.data.index),
            _ => return IgnoreNextHeaderDelimiterEvents::Illegal,
        };
        match char {
            Some(c) if c == &self.data.delimiter => IgnoreNextHeaderDelimiterEvents::FoundDelimiter,
            Some(c) if c == &b'\n' => IgnoreNextHeaderDelimiterEvents::FoundNewLine,
            _ => IgnoreNextHeaderDelimiterEvents::Illegal,
        }
    }
    fn ignore_next_body_delimiter(&self) -> IgnoreNextBodyDelimiterEvents {
        let char = match self.data.input {
            Some(input) if !input.is_empty() => input.as_bytes().get(self.data.index),
            _ => return IgnoreNextBodyDelimiterEvents::Illegal,
        };
        match char {
            Some(c) if c == &self.data.delimiter => IgnoreNextBodyDelimiterEvents::FoundDelimiter,
            Some(c) if c == &b'\n' => IgnoreNextBodyDelimiterEvents::FoundNewLine,
            _ => IgnoreNextBodyDelimiterEvents::Illegal,
        }
    }
}

impl<'a> CsvParser<'a> {
    pub fn new(delimiter: char, trim_quotes: bool) -> Self {
        let mut parser = CsvParser::default();
        parser.data.delimiter = delimiter as u8;
        parser.data.trim_quotes = trim_quotes;
        parser
    }
    pub fn parse(&mut self, text: &'a String) -> Result<Option<CSVData>, Box<dyn Error>> {
        self.data.input = Some(text);
        self.run()?;
        Ok(self.data.parsed_csv.take())
    }
}
